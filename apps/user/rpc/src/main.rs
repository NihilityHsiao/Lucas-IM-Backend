use tracing::Level;
use user_rpc::config::Config;
use user_rpc::UserRpcServer;

const CONFIG_PATH: &str = "./apps/user/rpc/etc/user.yml";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let config = Config::new(CONFIG_PATH);

    UserRpcServer::start(config)
        .await
        .expect("Failed to start server");

    Ok(())
}
