use crate::service_register::{ServiceInstance, ServiceRegister};
use crate::{EtcdConfig, ETCD_NAMESPACE};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use etcd_client::{GetOptions, LeaseKeepAliveStream, LeaseKeeper};
use nanoid::nanoid;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use tracing::{debug, error, info, warn};

type LeaseId = i64;
const DEFAULT_TTL_SECOND: i64 = 60;
const DEFAULT_KEEPALIVE_INTERVAL_SECOND: u64 = 30;

struct Options {
    namespace: String,
    ttl: i64, // second
    max_retry: i32,
}

pub struct EtcdServiceRegister {
    options: Options,
    client: etcd_client::Client,
    kv: etcd_client::KvClient,
    lease_id: i64,
}

impl EtcdServiceRegister {
    pub async fn new(client: etcd_client::Client) -> anyhow::Result<Self> {
        let op = Options {
            namespace: ETCD_NAMESPACE.to_string(),
            ttl: DEFAULT_TTL_SECOND * 2,
            max_retry: 5,
        };

        let kv = client.kv_client().clone();
        let reg = EtcdServiceRegister {
            options: op,
            client,
            kv,
            lease_id: LeaseId::default(),
        };
        Ok(reg)
    }
    pub async fn from_config(config: &EtcdConfig) -> anyhow::Result<Self> {
        let options = etcd_client::ConnectOptions::default();
        let client = etcd_client::Client::connect(&config.hosts, Some(options))
            .await
            .map_err(|e| anyhow!("connect to etcd failed: {}", e.to_string()))?;
        let op = Options {
            namespace:ETCD_NAMESPACE.to_string(),
            ttl: DEFAULT_TTL_SECOND,
            max_retry: 5,
        };
        let kv = client.kv_client().clone();

        Ok(Self {
            client,
            options: op,
            kv,
            lease_id: 0,
        })
    }

    async fn lease_grant_and_keepalive(
        &mut self,
        ttl_second: i64,
        keep_alive_interval: u64,
    ) -> anyhow::Result<LeaseId> {
        let resp = self.client.lease_grant(ttl_second, None).await?;
        self.lease_id = resp.id();
        if keep_alive_interval > 0 {
            self.keep_alive(keep_alive_interval).await?;
        }

        Ok(resp.id())
    }

    async fn keep_alive(&mut self, keep_alive_interval: u64) -> anyhow::Result<()> {
        info!("keepalive interval: {}", keep_alive_interval);
        let mut client = self.client.clone();
        let lease_id = self.lease_id;

        tokio::spawn(async move {
            loop {
                let (mut lease_keeper, mut lease_keep_alive_stream) = loop {
                    let keeper = client.lease_keep_alive(lease_id).await;
                    if keeper.is_err() {
                        warn!(
                            "etcd client lease keep alive error: {}",
                            keeper.unwrap_err()
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        continue;
                    }
                    let (lease_keeper, lease_keep_alive_stream) = keeper.unwrap();

                    break (lease_keeper, lease_keep_alive_stream);
                };

                let mut retry: u8 = 0;
                const MAX_RETRY: u8 = 3;

                let lease_result = Self::keep_alive_loop(
                    lease_id,
                    keep_alive_interval,
                    lease_keeper,
                    lease_keep_alive_stream,
                )
                .await;
                match lease_result {
                    Ok(_) => {
                        error!("lease keep alive loop exit");
                    }
                    Err(_) => {
                        if retry >= MAX_RETRY {
                            error!("lease keep alive max retry reached({})", MAX_RETRY);
                            break;
                        } else {
                            retry += 1;
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn keep_alive_loop(
        lease_id: LeaseId,
        interval_second: u64,
        mut lease_keeper: LeaseKeeper,
        mut lease_keep_alive_stream: LeaseKeepAliveStream,
    ) -> anyhow::Result<()> {
        loop {
            let _ = lease_keeper.keep_alive().await;
            let resp = lease_keep_alive_stream.message().await;
            match resp {
                Ok(resp) => {
                    match resp {
                        None => {
                            warn!("lease keep alive stream closed");
                            bail!("lease keep alive stream closed")
                        }
                        Some(resp) => {
                            if resp.ttl() <= 0 {
                                warn!("leaseId {:?} expired", lease_id);
                                bail!("leaseId {:?} expired", lease_id);
                            }
                            //  只有这种情况是续约成功
                            debug!(
                                "leaseId {:?} keep alive success, new ttl: {}",
                                lease_id,
                                resp.ttl()
                            );
                        }
                    }
                }
                Err(e) => {
                    warn!("lease keep alive error: {:?}", e);
                    bail!("lease keep alive error: {:?}", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(interval_second)).await;
        }
    }
}

impl Debug for EtcdServiceRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EtcdServiceRegister {{ lease_id: {} }}", self.lease_id)
    }
}

impl Drop for EtcdServiceRegister {
    fn drop(&mut self) {
        if self.lease_id > 0 {
            let lease_id = self.lease_id;
            let mut client = self.client.lease_client();
            tokio::spawn(async move {
                info!("drop etcd register leaseId: {}", lease_id);
                client.revoke(lease_id).await.unwrap();
            });
        }
    }
}

#[async_trait]
impl ServiceRegister for EtcdServiceRegister {
    async fn register(&mut self, service: ServiceInstance) -> anyhow::Result<()> {
        let key = format!("{}/{}/{}", self.options.namespace, service.name, service.id);

        let value = serde_json::to_string(&service)?;
        if self.lease_id != 0 {
            self.client.lease_revoke(self.lease_id).await?;
        }

        let lease_id = self
            .lease_grant_and_keepalive(self.options.ttl, DEFAULT_KEEPALIVE_INTERVAL_SECOND)
            .await
            .map_err(|e| anyhow!("etcd lease grant failed: {}", e.to_string()))?;
        self.lease_id = lease_id;

        let opt = Some(etcd_client::PutOptions::new().with_lease(self.lease_id));

        let _ = self
            .client
            .put(key, value, opt)
            .await
            .map_err(|e| anyhow!("etcd register error: {}", e))?;

        Ok(())
    }

    async fn unregister(&mut self) -> anyhow::Result<()> {
        if self.lease_id > 0 {
            self.client.lease_revoke(self.lease_id).await?;
        }

        Ok(())
    }

    async fn get_service(&mut self, name: &str) -> anyhow::Result<Vec<ServiceInstance>> {
        let key = format!("{}/{}", self.options.namespace, name);
        let opt = GetOptions::new().with_prefix();
        let resp = self.kv.get(key, Some(opt)).await?;

        let kvs = resp.kvs();
        let mut services = vec![];
        for kv in kvs {
            let value = kv.value_str().unwrap_or_default();

            if value.is_empty() {
                continue;
            }

            let service: ServiceInstance = serde_json::from_str(value)?;
            services.push(service);
        }
        Ok(services)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_service() -> anyhow::Result<()> {
        let etcd_config = EtcdConfig {
            hosts: vec!["192.168.0.103:2379".to_string()],
            key: "user.rpc".to_string(),
        };
        let mut reg_1 = EtcdServiceRegister::from_config(&etcd_config).await?;

        let services = reg_1.get_service("user").await;

        println!("services: {:#?}", services);

        Ok(())
    }

    #[tokio::test]
    async fn test_etcd_register() -> anyhow::Result<()> {
        let config = EtcdConfig {
            hosts: vec!["192.168.0.103:2379".to_string()],
            key: "user.rpc".to_string(),
        };
        let mut reg_1 = EtcdServiceRegister::from_config(&config).await?;

        let service_1 = ServiceInstance {
            id: nanoid!(),
            endpoints: vec![
                "192.168.0.6:20001".to_string(),
                "192.168.0.6:20002".to_string(),
            ],
            name: "user-rpc".to_string(),
            version: "0.1".to_string(),
            metadata: Default::default(),
        };

        reg_1.register(service_1).await?;

        let mut reg_2 = EtcdServiceRegister::from_config(&config).await?;

        let service_2 = ServiceInstance {
            id: nanoid!(),
            endpoints: vec![
                "192.168.0.6:20001".to_string(),
                "192.168.0.6:20002".to_string(),
                "192.168.0.6:20003".to_string(),
            ],
            name: "user-rpc".to_string(),
            version: "0.1".to_string(),
            metadata: Default::default(),
        };
        reg_2.register(service_2).await?;

        Ok(())
    }
}
