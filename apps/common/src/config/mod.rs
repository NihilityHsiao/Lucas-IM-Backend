use std::path::Path;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub trait LoadableConfig:Sized + DeserializeOwned{
     fn load(file: impl AsRef<Path>) -> Self {
         let content = std::fs::read_to_string(file).expect("load config file success");
         let config: Self = serde_yaml::from_str(&content).expect("parse config file success");
         config
     }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EtcdConfig {
    pub hosts: Vec<String>,
    pub key: String,
    pub scheme: String,
}

impl  EtcdConfig {
    pub fn urls(&self) -> Vec<String> {
        self.hosts.iter().map(|s| format!("{}://{}", self.scheme, s)).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoDbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}
impl PostgresConfig {
    fn server_url(&self) -> String {
        if self.password.is_empty() {
            return format!("postgres://{}@{}:{}", self.user, self.host, self.port);
        }
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }
    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url(), self.database)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}
impl RedisConfig {
    pub fn url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_expire: u64,
}
