use crate::pb;
use crate::pb::user::{UserOnlineCountRequest, UserOnlineCountResponse};
use crate::service_context::ServiceContext;
use tracing::info;

pub async fn get_user_online_count_logic(
    svc: &ServiceContext,
    request: tonic::Request<UserOnlineCountRequest>,
) -> Result<tonic::Response<UserOnlineCountResponse>, tonic::Status> {
    info!("request: {:?}", request);
    let count = svc.cache.get_user_online_count().await?;

    Ok(tonic::Response::new(UserOnlineCountResponse { count }))
}
