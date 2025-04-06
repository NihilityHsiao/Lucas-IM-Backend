use crate::service_discovery::ServiceDiscovery;
use crate::service_register::{ServiceInstance, ServiceRegister};
use dashmap::DashMap;
use std::fmt::Formatter;
use std::sync::Arc;
use async_trait::async_trait;
use tonic::transport::Channel;

pub struct EtcdServiceDiscovery {
    client: etcd_client::Client,
    service_map: DashMap<String, Channel>,
    service_center: Arc<dyn ServiceRegister>,
    discovery_internal_second: i64,
}

impl std::fmt::Debug for EtcdServiceDiscovery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[async_trait]
impl ServiceDiscovery for EtcdServiceDiscovery {
    async fn get_service(&self, service_name: &str) -> anyhow::Result<Vec<ServiceInstance>> {
        todo!()
    }
}
