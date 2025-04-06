use common::LoadableConfig;
use tracing::Level;
use user_api::config::Config;
use user_api::start_server;

const CONFIG_PATH: &str = "./apps/user/api/etc/user.yml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(Level::INFO)
        .init();
    let config = Config::load(CONFIG_PATH);
    start_server(config).await;

    Ok(())
}
