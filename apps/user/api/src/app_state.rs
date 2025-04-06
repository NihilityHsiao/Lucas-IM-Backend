use crate::config::Config;
use common::service_discovery::etcd::EtcdServiceDiscovery;
use std::sync::Arc;
use user_rpc::pb::user::user_service_client::UserServiceClient;

#[derive(Clone, Debug)]
pub struct AppState {
    pub service_discovery: Arc<EtcdServiceDiscovery>,
    pub user_rpc: UserServiceClient<tonic::transport::Channel>,
}

async fn init_etcd_discovery(config: &Config) -> EtcdServiceDiscovery {

    let etcd_client = etcd_client::Client::connect(config.user_rpc.etcd.hosts.clone(), None)
        .await
        .expect("Failed to connect to etcd.hosts.host");


    EtcdServiceDiscovery::new(etcd_client)
}

async fn init_user_rpc_client(config: &Config,  discovery:&mut EtcdServiceDiscovery) -> UserServiceClient<tonic::transport::Channel> {
    discovery
        .discovery(config.user_rpc.etcd.key.as_ref())
        .await
        .expect("Failed to discovery etcd.key");


    let user_rpc_channel = discovery
        .get_service_channel(config.user_rpc.etcd.key.as_ref())
        .await
        .expect("get service channel");

        UserServiceClient::new(user_rpc_channel)
}

impl AppState {
    pub async fn new(config: &Config) -> Self {

        let mut discovery = init_etcd_discovery(config).await;
        let user_rpc = init_user_rpc_client(config, &mut discovery).await;



        Self {
            service_discovery: Arc::new(discovery),
            user_rpc,
        }
    }
}
