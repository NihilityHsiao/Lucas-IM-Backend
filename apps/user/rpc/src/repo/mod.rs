use crate::error::Error;

use crate::pb::user::User;
use async_trait::async_trait;
use std::fmt::Debug;

pub(crate) mod postgres;
pub(crate) mod redis;

#[async_trait]
pub trait UserRepo: Sync + Send + Debug {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Error>;
    async fn find_by_account(&self, account: &str) -> Result<Option<User>, Error>;
    async fn find_by_account_or_email(
        &self,
        account: &str,
        email: &str,
    ) -> Result<Option<User>, Error>;
    async fn insert(&self, user: User) -> Result<(), Error>;

    async fn delete(&self, id: &str) -> Result<(), Error>;
}

#[async_trait]
pub trait Cache: Sync + Send + Debug {
    /// 获取用户临时验证码
    async fn get_user_register_code(&self, account: &str) -> Result<String, Error>;
    /// 保存用户临时验证码
    async fn save_user_register_code(&self, account: &str, code: &str) -> Result<(), Error>;
    /// 删除用户临时验证码
    async fn delete_user_register_code(&self, account: &str) -> Result<(), Error>;
    /// 设置用户登录状态
    async fn set_user_login(&self, user_id: &str) -> Result<(), Error>;
    /// 统计在线人数
    async fn get_user_online_count(&self) -> Result<i64, Error>;
}
