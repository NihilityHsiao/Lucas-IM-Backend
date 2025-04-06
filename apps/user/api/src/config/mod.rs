use common::{EtcdConfig, JwtConfig, LoadableConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub listen_on: String,
    pub user_rpc: RpcConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcConfig {
    pub etcd: EtcdConfig,
}

impl LoadableConfig for Config {}
