use async_trait::async_trait;
use std::fmt::Debug;
use tonic::transport::Channel;

pub mod etcd;

#[async_trait]
pub trait ServiceDiscovery: Send + Sync + Debug {
    async fn get_service(&self, service_name: &str) -> anyhow::Result<Channel>;
}
