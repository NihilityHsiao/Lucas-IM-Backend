use crate::pb;
use crate::service_context::ServiceContext;
use tracing::info;

pub async fn ping_logic(
    ctx: &ServiceContext,
    request: tonic::Request<pb::user::Request>,
) -> Result<tonic::Response<pb::user::Response>, tonic::Status> {
    info!("request: {:?}", request);
    let resp = pb::user::Response {
        pong: "pong".to_string(),
    };

    Ok(tonic::Response::new(resp))
}
