use crate::error::Error;
use crate::pb::user::{RegisterRequest, RegisterResponse, User};
use crate::service_context::ServiceContext;
use crate::utils;
use nanoid::nanoid;
use tonic::{Request, Response, Status};
use tracing::info;

pub async fn register_logic(
    svc: &ServiceContext,
    request: Request<RegisterRequest>,
) -> Result<Response<RegisterResponse>, Status> {
    info!("request: {:?}", request);
    // todo: 校验验证码, 用户是否存在, 密码加密, 保存用户信息到数据库, 生成token, 返回token
    let req = request.into_inner();

    if req.account.is_empty() {
        return Err(Status::from(Error::invalid_account("account is empty")));
    }

    // 检查邮箱/账号是否已经注册
    let user = svc
        .user_repo
        .find_by_account_or_email(&req.account, &req.email)
        .await?;
    if user.is_some() {
        let user = user.unwrap();
        if user.account == req.account {
            return Err(Status::from(Error::invalid_account(
                "account already exists",
            )));
        }

        if user.email == Option::from(req.email.clone()) {
            return Err(Status::from(Error::invalid_email("email already exists")));
        }
    }

    let code = match svc.cache.get_user_register_code(&req.account).await {
        Ok(code) => code,
        Err(_) => {
            return Err(Status::from(Error::invalid_code("code expired")));
        }
    };

    if code != req.code {
        return Err(Status::from(Error::invalid_code("code mismatch")));
    }

    // encode password
    let salt = utils::generate_salt();
    let encoded_password = utils::hash_password(req.password.as_bytes(), &salt)?;

    let user = User {
        id: nanoid!(),
        name: req.name,
        account: req.account,
        password: encoded_password,
        avatar: req.avatar,
        email: Some(req.email),
        birthday: None,
        salt,
        ..Default::default()
    };

    svc.user_repo.insert(user).await?;

    // todo: 生成token

    let resp = RegisterResponse::default();
    Ok(Response::new(resp))
}
