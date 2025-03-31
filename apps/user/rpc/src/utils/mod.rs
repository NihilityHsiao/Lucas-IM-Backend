use crate::error::Error;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

pub(crate) mod jwt;

pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

/// 使用Argon2算法对密码进行安全哈希处理
///
/// 该函数使用加密安全的盐值和Argon2id算法对用户密码进行哈希，
/// 提供抗暴力破解和彩虹表攻击的保护。
///
/// # 参数
/// - `password`: 用户原始密码字节数组
/// - `salt`: 加密盐值（建议使用generate_salt生成）
///
/// # 返回值
/// - `Result<String, Error>`: 成功返回PHC格式的哈希字符串，失败返回错误信息
pub fn hash_password(password: &[u8], salt: &str) -> Result<String, Error> {
    // 使用默认的Argon2配置
    // 这个配置可以更改为适合您具体安全需求和性能要求的设置

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password, &SaltString::from_b64(salt).unwrap())
        .map_err(|e| Error::internal_with_details(e.to_string()))?
        .to_string())
}
