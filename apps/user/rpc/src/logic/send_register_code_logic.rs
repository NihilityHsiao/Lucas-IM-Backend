use crate::error::Error;
use crate::pb::user::{SendRegisterCodeRequest, SendRegisterCodeResponse};
use crate::service_context::ServiceContext;
use rand::Rng;
use tonic::{Request, Response, Status};
use tracing::info;

pub async fn send_register_code_logic(
    svc: &ServiceContext,
    request: Request<SendRegisterCodeRequest>,
) -> Result<Response<SendRegisterCodeResponse>, Status> {
    let req = request.into_inner();
    info!("request: {:?}", req);

    if req.account.is_empty() {
        return Err(Status::from(Error::invalid_account("account is empty")));
    }

    if req.email.is_empty() {
        return Err(Status::from(Error::invalid_account("email is empty")));
    }

    // 生成验证码,存入缓存
    let mut code = svc
        .cache
        .get_user_register_code(&req.account)
        .await
        .unwrap_or_default();
    if code.is_empty() {
        let num = {
            let mut rng = rand::thread_rng();
            rng.gen_range(1000..5000)
        };
        code = num.to_string();
        svc.cache
            .save_user_register_code(&req.account, &code)
            .await?;
    }

    // todo: 发送验证码到邮箱

    Ok(Response::new(SendRegisterCodeResponse { code }))
}
