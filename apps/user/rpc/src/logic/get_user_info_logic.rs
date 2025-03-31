use crate::pb;
use crate::pb::user::{
    GetUserInfoRequest, GetUserInfoResponse, UserOnlineCountRequest, UserOnlineCountResponse,
};
use crate::service_context::ServiceContext;
use tracing::info;

pub async fn get_user_info_logic(
    svc: &ServiceContext,
    request: tonic::Request<GetUserInfoRequest>,
) -> Result<tonic::Response<GetUserInfoResponse>, tonic::Status> {
    info!("request: {:?}", request);
    let user_id = request.into_inner().user_id;
    let user = svc.user_repo.find_by_id(&user_id).await?;
    Ok(tonic::Response::new(GetUserInfoResponse { user }))
}
