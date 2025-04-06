use common::service_discovery::etcd::EtcdServiceDiscovery;
use common::service_register::ServiceRegister;
use tracing::Level;
use common::LoadableConfig;
use user_rpc::config::Config;
use user_rpc::UserRpcServer;

const CONFIG_PATH: &str = "./apps/user/rpc/etc/user.yml";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let config = Config::load(CONFIG_PATH);
    println!("config: {:?}", config);

    UserRpcServer::start(config)
        .await
        .expect("Failed to start server");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::transport::Endpoint;
    use user_rpc::pb::user::user_service_client::UserServiceClient;
    use user_rpc::pb::user::{FindUserRequest, Request};

    #[tokio::test]
    async fn test_discover_user_rpc() -> anyhow::Result<()> {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();

        let mut config =
            Config::load(r"D:\SVN\Code\rust\Lucas-IM-Backend\apps\user\rpc\etc\user.yml");

        let etcd_client = etcd_client::Client::connect(config.etcd.hosts.clone(), None)
            .await
            .expect("Failed to connect to etcd.hosts.host");
        let mut discovery = EtcdServiceDiscovery::new(etcd_client);
        discovery.discovery("user.rpc").await.expect("success");

        let arc = discovery.get_service_map();
        let service_map = arc.read().unwrap();
        println!("service_map: {:?}", service_map);

        let user_channel = discovery
            .get_service_channel("user.rpc")
            .await
            .expect("success");

        let mut user_rpc = UserServiceClient::new(user_channel);

        let x = user_rpc
            .find_user(FindUserRequest {
                user_id: vec!["xcTZJv2aWBX6QCSe0beNQ".to_string()],
                name: None,
                account: None,
                phone: None,
                email: None,
            })
            .await;

        let user = x.unwrap().into_inner();

        println!("user: {:?}", user.users);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_rpc_client() -> anyhow::Result<()> {
        let mut endpoints = vec!["http://localhost:50051"];

        let endpoint = Endpoint::from_static("http://localhost:50051");
        let mut client = UserServiceClient::connect(endpoint).await?;

        let x = client.ping(Request::default()).await;
        assert!(x.is_ok());

        Ok(())
    }
}
