use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod etcd;


#[derive(Debug,Serialize,Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub endpoints: Vec<String>,
    pub name: String,
    pub version: String,
    pub metadata: HashMap<String, String>,
}


#[async_trait]
pub trait ServiceRegister {
    async fn register(&mut self, registration: ServiceInstance) -> anyhow::Result<()>;
    async fn unregister(&mut self) -> anyhow::Result<()>;
}

