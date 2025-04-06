use crate::pb::user::{FindUserRequest, FindUserResponse};
use crate::service_context::ServiceContext;
use tracing::info;

pub async fn find_user_logic(
    svc: &ServiceContext,
    request: tonic::Request<FindUserRequest>,
) -> Result<tonic::Response<FindUserResponse>, tonic::Status> {
    info!("request: {:?}", request);

    let req = request.into_inner();
    let mut users = vec![];

    if req.user_id.len() > 0 {
        let user = svc.user_repo.find_by_ids(req.user_id).await?;
        users.extend(user);
    }
    if req.name.is_some() {
        let user = svc.user_repo.find_by_name(&req.name.unwrap()).await?;
        if let Some(user) = user {
            users.push(user);
        }
    }

    if req.account.is_some() {
        let user = svc.user_repo.find_by_account(&req.account.unwrap()).await?;
        if let Some(user) = user {
            users.push(user);
        }
    }

    if req.phone.is_some() {
        let user = svc.user_repo.find_by_phone(&req.phone.unwrap()).await?;
        if let Some(user) = user {
            users.push(user);
        }
    }

    if req.email.is_some() {
        let user = svc.user_repo.find_by_email(&req.email.unwrap()).await?;
        if let Some(user) = user {
            users.push(user);
        }
    }

    Ok(tonic::Response::new(FindUserResponse { users }))
}
