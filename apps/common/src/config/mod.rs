use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EtcdConfig {
    pub hosts: Vec<String>,
    pub key: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_expire: u64,
}
