
use dashmap::DashMap;
use std::fmt::Formatter;
use tonic::transport::Channel;

pub struct EtcdServiceDiscovery {
    client: etcd_client::Client,
    service_map: DashMap<String, Channel>,
}

impl std::fmt::Debug for EtcdServiceDiscovery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
