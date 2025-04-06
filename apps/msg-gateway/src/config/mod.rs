use serde::{Deserialize, Serialize};
use common::{EtcdConfig, RedisConfig};

#[derive(Debug,Serialize,Deserialize)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) listen_on: String,
    pub(crate) etcd: EtcdConfig,
    pub(crate) redis: RedisConfig,
}
