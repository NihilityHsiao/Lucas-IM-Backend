use serde::{Deserialize, Serialize};
use common::{EtcdConfig, JwtConfig, LoadableConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub listen_on: String,
    pub user_rpc: EtcdConfig,
    pub jwt: JwtConfig,
}

impl LoadableConfig for Config {}