use common::{EtcdConfig, JwtConfig, LoadableConfig, MongoDbConfig, PostgresConfig, RedisConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub listen_on: String,
    pub etcd: EtcdConfig,
    pub mongodb: MongoDbConfig,
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
}

impl LoadableConfig for Config {}
