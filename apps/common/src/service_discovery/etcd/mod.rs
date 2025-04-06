
use crate::service_discovery::ServiceDiscovery;

pub struct EtcdServiceDiscovery {
    client:etcd_client::Client,

}
impl EtcdServiceDiscovery {}

impl ServiceDiscovery for EtcdServiceDiscovery {}