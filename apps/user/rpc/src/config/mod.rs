use common::{EtcdConfig, JwtConfig, MongoDbConfig, PostgresConfig, RedisConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

impl Config {
    pub fn new(file: impl AsRef<Path>) -> Self {
        let content = std::fs::read_to_string(file).expect("load config file success");
        let config: Config = serde_yaml::from_str(&content).expect("parse config file success");

        config
    }
}
