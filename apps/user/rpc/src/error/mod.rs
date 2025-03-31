use std::error::Error as StdError;
mod tonic;

#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    UserNotFound,
    DbError,
    RedisError,
    // 无效验证码
    InvalidCode,
    // 无效账号
    InvalidAccount,

    // 账号或密码错误,
    InvalidAccountOrPassword,

    InternalError,
    // 未知错误
    Unknown,
    InvalidEmail,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    details: Option<String>,
    source: Option<Box<dyn StdError + Send + Sync>>,
}

impl Error {}

impl Error {
    pub fn new(
        kind: ErrorKind,
        details: impl Into<String>,
        source: impl StdError + 'static + Send + Sync,
    ) -> Self {
        Error {
            kind,
            source: Some(Box::new(source)),
            details: Some(details.into()),
        }
    }

    #[inline]
    pub fn with_details(kind: ErrorKind, details: impl Into<String>) -> Self {
        Self {
            kind,
            source: None,
            details: Some(details.into()),
        }
    }

    #[inline]
    pub fn internal_with_details(details: impl Into<String>) -> Self {
        Self::with_details(ErrorKind::InternalError, details)
    }

    pub fn invalid_code(details: impl Into<String>) -> Self {
        Self::with_details(ErrorKind::InvalidCode, details)
    }

    pub fn invalid_account(details: impl Into<String>) -> Self {
        Self::with_details(ErrorKind::InvalidAccount, details)
    }

    pub fn invalid_email(details: impl Into<String>) -> Self {
        Self::with_details(ErrorKind::InvalidEmail, details)
    }

    pub fn invalid_account_or_password(details: impl Into<String>) -> Self {
        Self::with_details(ErrorKind::InvalidAccountOrPassword, details)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        let error_kind = match value {
            sqlx::Error::RowNotFound => ErrorKind::UserNotFound,
            _ => ErrorKind::DbError,
        };

        Error::new(error_kind, value.to_string(), value)
    }
}

impl From<redis::RedisError> for Error {
    fn from(value: redis::RedisError) -> Self {
        Self::new(ErrorKind::RedisError, value.to_string(), value)
    }
}
