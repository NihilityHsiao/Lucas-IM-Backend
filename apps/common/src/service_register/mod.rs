use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

pub mod etcd;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub endpoints: Vec<String>,
    pub version: String,
    pub metadata: HashMap<String, String>,
}

#[async_trait]
pub trait ServiceRegister: Send + Sync + Debug {
    async fn register(&mut self, registration: ServiceInstance) -> anyhow::Result<()>;
    async fn unregister(&mut self) -> anyhow::Result<()>;

    async fn get_service(&mut self, name: &str) -> anyhow::Result<Vec<ServiceInstance>>;
}
