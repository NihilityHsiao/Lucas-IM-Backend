use crate::config::Config;
use crate::error::Error;

use crate::pb::user::User;
use crate::repo::UserRepo;
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use std::fmt::Debug;

#[derive(Debug)]
pub struct UserPostgres {
    pool: PgPool,
}

impl UserPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn from_config(config: &Config) -> Self {
        let pool = PgPool::connect(&config.postgres.url())
            .await
            .expect("connect to postgres success");
        Self::new(pool)
    }
}

#[async_trait]
impl UserRepo for UserPostgres {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as("select * from users where id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_account(&self, account: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as("select * from users where account = $1")
            .bind(account)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_account_or_email(
        &self,
        account: &str,
        email: &str,
    ) -> Result<Option<User>, Error> {
        let user = sqlx::query_as("select * from users where account = $1 or email = $2")
            .bind(account)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn insert(&self, user: User) -> Result<(), Error> {
        let now = chrono::Utc::now().timestamp_millis();
        let mut tx = self.pool.begin().await?;
        let result = sqlx::query_as(
            "INSERT INTO users
            (id, name, account, password, avatar, gender, age, phone, email, address, region, salt, signature, create_time, update_time)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) RETURNING *")
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.account)
            .bind(&user.password)
            .bind(&user.avatar)
            .bind(&user.gender)
            .bind(user.age)
            .bind(&user.phone)
            .bind(&user.email)
            .bind(&user.address)
            .bind(&user.region)
            .bind(&user.salt)
            .bind(&user.signature)
            .bind(now)
            .bind(now)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(result)
    }

    async fn delete(&self, id: &str) -> Result<(), Error> {
        sqlx::query("delete from users where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        // let _ = sqlx::query("UPDATE users SET is_delete = TRUE WHERE id = $1")
        //     .bind(id)
        //     .execute(&self.pool)
        //     .await?;
        Ok(())
    }
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            account: row.try_get("account")?,
            password: row.try_get("password")?,
            avatar: row.try_get("avatar")?,
            gender: row.try_get("gender")?,
            age: row.try_get("age")?,
            phone: row.try_get("phone")?,
            email: row.try_get("email")?,
            address: row.try_get("address")?,
            region: row.try_get("region")?,
            birthday: row.try_get("birthday")?,
            create_time: row.try_get("create_time")?,
            update_time: row.try_get("update_time")?,
            salt: row.try_get("salt")?,
            signature: row.try_get("signature")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use nanoid::nanoid;

    #[tokio::test]
    async fn test_find_by_account() -> anyhow::Result<()> {
        let config = Config::new(r"etc/user.yml");
        let user_repo = UserPostgres::from_config(&config).await;
        let account = nanoid!();
        let user = User {
            id: nanoid!(),
            name: "test-name".to_string(),
            account: account.clone(),
            password: "123".to_string(),
            avatar: "123.png".to_string(),
            ..Default::default()
        };
        // 插入新用户
        user_repo
            .insert(user.clone())
            .await
            .expect("insert user success");

        // 查找用户
        let find_user = user_repo
            .find_by_account(&account)
            .await
            .expect("find user success");
        assert!(find_user.is_some());
        let find_user = find_user.unwrap();
        assert_eq!(user.id, find_user.id);
        assert_eq!(user.name, find_user.name);
        assert_eq!(user.account, find_user.account);
        assert_eq!(user.password, find_user.password);
        assert_eq!(user.avatar, find_user.avatar);

        // 删除用户
        let result = user_repo.delete(&account).await;
        assert!(result.is_ok());

        // 无法查找已删除的用户
        let find_user = user_repo
            .find_by_id(&account)
            .await
            .expect("find user success");
        assert!(find_user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_and_find_by_id_and_delete() -> anyhow::Result<()> {
        let config = Config::new(r"etc/user.yml");
        let user_repo = UserPostgres::from_config(&config).await;
        let id = nanoid!();
        let user = User {
            id: id.clone(),
            name: "test-name".to_string(),
            account: "test-account".to_string(),
            password: "123".to_string(),
            avatar: "123.png".to_string(),
            ..Default::default()
        };
        // 插入新用户
        user_repo
            .insert(user.clone())
            .await
            .expect("insert user success");

        // 查找用户
        let find_user = user_repo.find_by_id(&id).await.expect("find user success");
        assert!(find_user.is_some());
        let find_user = find_user.unwrap();
        assert_eq!(user.id, find_user.id);
        assert_eq!(user.name, find_user.name);
        assert_eq!(user.account, find_user.account);
        assert_eq!(user.password, find_user.password);
        assert_eq!(user.avatar, find_user.avatar);

        // 删除用户
        let result = user_repo.delete(&id).await;
        assert!(result.is_ok());

        // 无法查找已删除的用户
        let find_user = user_repo.find_by_id(&id).await.expect("find user success");
        assert!(find_user.is_none());

        Ok(())
    }
}
