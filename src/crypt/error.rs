use serde::Serialize;
use thiserror::Error;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("Failed to creat Hmac Key")]
    KeyFailHmac,
    #[error("Password is not valid")]
    PasswordNotValid,
}
