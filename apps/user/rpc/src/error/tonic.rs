use crate::error::{Error, ErrorKind};
use tonic::Status;
use tracing::info;

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        let code = match value.kind {
            ErrorKind::UserNotFound | ErrorKind::NotFound => tonic::Code::NotFound,
            ErrorKind::DbError
            | ErrorKind::InvalidAccountOrPassword
            | ErrorKind::RedisError
            | ErrorKind::Unknown
            | ErrorKind::InternalError => tonic::Code::Internal,
            ErrorKind::InvalidCode | ErrorKind::InvalidAccount | ErrorKind::InvalidEmail => {
                tonic::Code::InvalidArgument
            }
        };

        let kind = format!("{:?}", value.kind);
        let details = value.details.unwrap_or_default();
        info!("details: {:?}", details);

        Status::with_details(code, &kind, details.into())
    }
}

impl From<tonic::Status> for Error {
    fn from(value: Status) -> Self {
        let kind = match value.message() {
            "NotFound" => ErrorKind::NotFound,
            "UnknownError" => ErrorKind::Unknown,
            _ => ErrorKind::InternalError,
        };

        let details = match String::from_utf8(value.details().to_vec()) {
            Ok(details) => Some(details),
            Err(_) => None,
        };

        Self {
            kind,
            details,
            source: None,
        }
    }
}
