use common::service_register::etcd::EtcdServiceRegister;
use common::service_register::{ServiceInstance, ServiceRegister};
use nanoid::nanoid;
use tracing::Level;
use user_rpc::config::Config;
use user_rpc::UserRpcServer;

const CONFIG_PATH: &str = "./apps/user/rpc/etc/user.yml";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let config = Config::new(CONFIG_PATH);

    let mut reg = EtcdServiceRegister::from_config(&config.etcd).await?;
    let _ = reg
        .register(ServiceInstance {
            id: nanoid!(),
            name: config.name.clone(),
            endpoints: vec![config.listen_on.clone()],
            version: "0.1".to_string(),
            metadata: Default::default(),
        })
        .await;

    UserRpcServer::start(config)
        .await
        .expect("Failed to start server");

    Ok(())
}
