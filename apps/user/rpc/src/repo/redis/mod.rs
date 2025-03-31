use crate::config::Config;
use crate::error::Error;
use crate::repo::Cache;
use async_trait::async_trait;
use redis::AsyncCommands;

const REGISTER_CODE_KEY: &str = "register_code";
const REGISTER_CODE_TTL_SECONDS: i64 = 300;

const USER_ONLINE_SET: &str = "user_online_set";

#[derive(Debug)]
pub struct RedisCache {
    client: redis::Client,
}
impl RedisCache {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    pub fn from_config(config: &Config) -> Self {
        let client = redis::Client::open(config.redis.url()).expect("open redis client success");
        Self { client }
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get_user_register_code(&self, account: &str) -> Result<String, Error> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result = conn.hget(REGISTER_CODE_KEY, account).await?;
        Ok(result)
    }

    async fn save_user_register_code(&self, account: &str, code: &str) -> Result<(), Error> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let mut pipe = redis::pipe();
        pipe.hset(REGISTER_CODE_KEY, account, code)
            .expire(REGISTER_CODE_KEY, REGISTER_CODE_TTL_SECONDS)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn delete_user_register_code(&self, account: &str) -> Result<(), Error> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.hdel(REGISTER_CODE_KEY, account).await?;
        Ok(())
    }

    async fn set_user_login(&self, user_id: &str) -> Result<(), Error> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.sadd(USER_ONLINE_SET, user_id).await?;
        Ok(())
    }

    async fn get_user_online_count(&self) -> Result<i64, Error> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: i64 = conn.scard(USER_ONLINE_SET).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_cache_save_and_get_and_delete_user_register_code() -> anyhow::Result<()> {
        let config = Config::new("etc/user.yml");
        let cache = RedisCache::from_config(&config);

        let code = "123456";
        let account = "test_account";

        // save
        assert!(cache.get_user_register_code(account).await.is_ok());

        // get
        let get_res = cache.get_user_register_code(account).await;
        println!("get_res: {:?}", get_res);
        assert!(get_res.is_ok());
        let get_code = get_res.unwrap();
        println!("get code: {}", code);
        assert_eq!(get_code, code);

        // delete
        assert!(cache.delete_user_register_code(account).await.is_ok());

        // can not get after delete
        assert!(cache.get_user_register_code(account).await.is_err());

        Ok(())
    }
}
