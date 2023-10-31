use crate::model;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config missing env: {0}")]
    ConfigMissingEnv(&'static str),
    // -- Modules
    #[error("Model Error: {0:?}")]
    Model(#[from] model::Error),
    #[error("Config invalid value: {0}")]
    ConfigInvalidValue(&'static str),
    #[error("Config wrong format: {0}")]
    ConfigWrongFormat(&'static str),
}
// endregion: --- Error Boilerplate
