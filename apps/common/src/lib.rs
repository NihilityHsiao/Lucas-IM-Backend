use serde::{Deserialize, Serialize};

pub mod config;
pub mod service_discovery;
pub mod service_register;

pub use config::*;


pub(crate) const ETCD_NAMESPACE: &str = "/lucasim/services";

