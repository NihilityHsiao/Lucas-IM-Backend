use crate::config::Config;
use crate::repo::postgres::user::UserPostgres;
use crate::repo::redis::RedisCache;
use crate::repo::{Cache, UserRepo};

pub struct ServiceContext {
    pub config: Config,
    pub user_repo: Box<dyn UserRepo>,
    pub cache: Box<dyn Cache>,
}

impl ServiceContext {
    pub async fn new(config: Config) -> ServiceContext {
        let user_repo = Box::new(UserPostgres::from_config(&config).await);
        let cache = Box::new(RedisCache::from_config(&config));

        let ctx = ServiceContext {
            config,
            user_repo,
            cache,
        };

        ctx
    }
}
