use crate::error::Error;
use crate::pb::user::{LoginRequest, LoginResponse};
use crate::service_context::ServiceContext;
use crate::utils::jwt::gen_token;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use tonic::{Request, Response, Status};
use tracing::info;

pub async fn login_logic(
    svc: &ServiceContext,
    request: Request<LoginRequest>,
) -> Result<Response<LoginResponse>, Status> {
    let req = request.into_inner();

    let Some(user) = svc.user_repo.find_by_account(&req.account).await? else {
        return Err(Status::from(Error::invalid_account_or_password(
            "账号或密码错误",
        )));
    };

    let password_hash = match PasswordHash::new(&user.password) {
        Ok(password) => password,
        Err(e) => {
            return Err(Status::from(Error::internal_with_details(format!(
                "密码哈希错误 {}",
                e
            ))));
        }
    };

    let valid = Argon2::default()
        .verify_password(req.password.as_bytes(), &password_hash)
        .is_ok();
    if !valid {
        return Err(Status::from(Error::invalid_account_or_password(
            "账号或密码错误",
        )));
    }

    // 生成jwt token
    let token = gen_token(&user, &svc.config.jwt.secret)?;
    info!("gen token: {:?}", token);

    // 设置登录态
    svc.cache.set_user_login(&user.id).await?;

    Ok(Response::new(LoginResponse {
        user_id: token.user_id,
        token: token.token,
        refresh_token: token.refresh_token,
    }))
}
