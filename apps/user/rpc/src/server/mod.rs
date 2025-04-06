use crate::config::Config;
use crate::logic::{
    find_user_logic, get_user_info_logic, get_user_online_count_logic, login_logic, ping_logic,
    register_logic, send_register_code_logic,
};
use crate::pb;
use crate::pb::user::user_service_server::{UserService, UserServiceServer};
use crate::pb::user::{
    FindUserRequest, FindUserResponse, GetUserInfoRequest, GetUserInfoResponse, LoginRequest,
    LoginResponse, RegisterRequest, RegisterResponse, SendRegisterCodeRequest,
    SendRegisterCodeResponse, UserOnlineCountRequest, UserOnlineCountResponse,
};
use crate::service_context::ServiceContext;
use common::service_register::etcd::EtcdServiceRegister;
use common::service_register::{ServiceInstance, ServiceRegister};
use nanoid::nanoid;
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status};
use tracing::info;

pub struct UserRpcServer {
    svc: ServiceContext,
    service_register: EtcdServiceRegister,
}

impl UserRpcServer {
    async fn new(config: Config) -> UserRpcServer {
        let service_register = EtcdServiceRegister::from_config(&config.etcd)
            .await
            .expect("EtcdServiceRegister success");
        Self {
            svc: ServiceContext::new(config.clone()).await,
            service_register,
        }
    }

    pub async fn start(config: Config) -> anyhow::Result<()> {
        let mut user_service_rpc = UserRpcServer::new(config.clone()).await;
        // 注册服务发现
        user_service_rpc
            .service_register
            .register(ServiceInstance {
                id: nanoid!(),
                name: config.etcd.key.clone(),
                endpoints: vec![config.listen_on.clone()],
                version: "0.1".to_string(),
                metadata: Default::default(),
            })
            .await
            .expect("register success");

        let service = UserServiceServer::new(user_service_rpc);
        info!("listen on: {}", config.listen_on.clone());

        Server::builder()
            .add_service(service)
            .serve(
                config
                    .listen_on
                    .parse()
                    .expect("parse config listen on address success"),
            )
            .await
            .expect("serve start success");
        Ok(())
    }
}

#[async_trait]
impl UserService for UserRpcServer {
    async fn ping(
        &self,
        request: Request<pb::user::Request>,
    ) -> Result<Response<pb::user::Response>, Status> {
        ping_logic(&self.svc, request).await
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        register_logic(&self.svc, request).await
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        login_logic(&self.svc, request).await
    }

    async fn send_register_code(
        &self,
        request: Request<SendRegisterCodeRequest>,
    ) -> Result<Response<SendRegisterCodeResponse>, Status> {
        send_register_code_logic(&self.svc, request).await
    }

    async fn get_user_online_count(
        &self,
        request: Request<UserOnlineCountRequest>,
    ) -> Result<Response<UserOnlineCountResponse>, Status> {
        get_user_online_count_logic(&self.svc, request).await
    }

    async fn get_user_info(
        &self,
        request: Request<GetUserInfoRequest>,
    ) -> Result<Response<GetUserInfoResponse>, Status> {
        get_user_info_logic(&self.svc, request).await
    }

    async fn find_user(
        &self,
        request: Request<FindUserRequest>,
    ) -> Result<Response<FindUserResponse>, Status> {
        find_user_logic(&self.svc, request).await
    }
}
