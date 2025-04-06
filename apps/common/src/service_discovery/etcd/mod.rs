use crate::service_discovery::ServiceDiscovery;
use crate::service_register::ServiceInstance;
use crate::{EtcdConfig, ETCD_NAMESPACE};
use async_trait::async_trait;
use dashmap::DashMap;
use etcd_client::{EventType, GetOptions, KeyValue, WatchOptions};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tonic::transport::channel::Change;
use tonic::transport::{Channel, Endpoint};
use tracing::info;

pub struct EtcdServiceDiscovery {
    etcd_client: etcd_client::Client,
    service_map: Arc<RwLock<HashMap<String, Channel>>>,
    tonic_channel: Channel,
    // 监听服务变化的channel
    tx: Sender<Change<String, Endpoint>>,
}

impl EtcdServiceDiscovery {
    pub fn new(client: etcd_client::Client) -> Self {
        let (channel, tx) = Channel::balance_channel(1024);
        Self {
            etcd_client: client,
            service_map: Arc::new(RwLock::new(HashMap::new())),

            tonic_channel: channel,
            tx,
        }
    }

    pub async fn get_service_channel(&self, service_name: &str) -> Option<Channel> {
        let key = format!("{}/{}", ETCD_NAMESPACE, service_name);
        self.service_map.read().unwrap().get(&key).cloned()
    }

    /// 启动服务发现
    pub async fn discovery(&mut self, name: &str) -> anyhow::Result<()> {
        info!(
            "discovery start, namespace/service_name:{}/{}",
            ETCD_NAMESPACE, name
        );
        let service_name = format!("{}/{}", ETCD_NAMESPACE, name);
        let opt = Some(GetOptions::new().with_prefix());
        let resp = self.etcd_client.get(service_name.clone(), opt).await?;
        info!("resp = {:?}", resp);
        for kv in resp.kvs() {
            let key = kv.key_str().unwrap_or_default();
            let value = kv.value_str().unwrap_or_default();
            let service_instance: ServiceInstance = serde_json::from_str(value)?;
            info!("discovery key:{},value:{:?}", key, service_instance);
            self.add_service(key, value).await;
        }

        let opt = Some(WatchOptions::new().with_prefix());
        let (mut watcher, mut stream) = self.etcd_client.watch(service_name, opt).await?;
        let service_map = self.service_map.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            while let Some(resp) = stream.message().await.unwrap() {
                for event in resp.events() {
                    match event.event_type() {
                        EventType::Put => {
                            info!("etcd event[put]: {:?}", event.kv());
                            if let Some(kv) = event.kv() {
                                let key = kv.key_str().unwrap_or_default();
                                let value = kv.value_str().unwrap_or_default();
                                if key.is_empty() {
                                    continue;
                                }
                                let service_instance: ServiceInstance =
                                    match serde_json::from_str(value) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            info!("etcd event[put] parse value error: {}", e);
                                            continue;
                                        }
                                    };
                                Self::add_service_map(&tx, &service_map, key, service_instance)
                                    .await;
                            }
                        }
                        EventType::Delete => {
                            info!("etcd event[delete]: {:?}", event.kv());
                            if let Some(kv) = event.kv() {
                                let key = kv.key_str().unwrap_or_default();
                                info!("etcd event[delete] key=: {:?}", key);
                                Self::remove_service_map(&tx, &service_map, key);
                            }
                        }
                    }
                }
            }

            let _ = watcher.cancel().await.unwrap();
        });

        Ok(())
    }
}

impl EtcdServiceDiscovery {
    #[inline]
    fn new_endpoint(
        uri: impl Into<String>,
        timeout: u64,
    ) -> Result<Endpoint, tonic::transport::Error> {
        Ok(Endpoint::from_shared(uri.into())?.timeout(Duration::from_secs(timeout)))
    }

    pub async fn add_service(&self, key: impl AsRef<str>, value: &str) {
        let instance = match serde_json::from_str(value) {
            Ok(v) => v,
            Err(e) => {
                info!("etcd event[put] parse value error: {}", e);
                return;
            }
        };
        Self::add_service_map(&self.tx, &self.service_map, key.as_ref(), instance).await;
    }
    #[allow(unused_variables)]
    #[inline]
    async fn add_service_map(
        rx: &Sender<Change<String, Endpoint>>,
        service_map: &RwLock<HashMap<String, Channel>>,
        key: impl Into<String>,
        instance: ServiceInstance,
    ) {
        let key = format!("{}/{}", ETCD_NAMESPACE, instance.name);

        for ins in instance.endpoints.iter() {
            info!("add service map, key:{:?}, instance:{:?}", key, ins);

            let ins = format!("http://{}", ins);
            if let Ok(endpoint) = Self::new_endpoint(ins, 10) {
                let channel = endpoint.connect().await.expect("connect success");
                service_map.write().unwrap().insert(key.clone(), channel);
                rx.try_send(Change::Insert(key.clone(), endpoint)).unwrap();
            } else {
                info!("tonic endpoint connect error: {:?}", instance);
            }
        }
    }

    #[inline]
    fn remove_service_map(
        rx: &Sender<Change<String, Endpoint>>,
        service_map: &RwLock<HashMap<String, Channel>>,
        key: impl AsRef<str>,
    ) {
        service_map.write().unwrap().remove(key.as_ref());
        rx.try_send(Change::Remove(key.as_ref().into()))
            .expect("send remove event to channel success");
    }
}

impl EtcdServiceDiscovery {
    pub fn get_service_map(&self) -> Arc<RwLock<HashMap<String, Channel>>> {
        self.service_map.clone()
    }

    pub fn get_etcd_client(&self) -> etcd_client::Client {
        self.etcd_client.clone()
    }
}

impl std::fmt::Debug for EtcdServiceDiscovery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[async_trait]
impl ServiceDiscovery for EtcdServiceDiscovery {
    async fn get_service(&self, service_name: &str) -> anyhow::Result<Channel> {
        todo!()
    }
}
