use thiserror::Error;

#[derive(Error, Debug)]
pub enum RError {
    #[error("Redis Error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Lettre Error: {0}")]
    LettreError(#[from] lettre::error::Error),
    #[error("LettreSmtp Error: {0}")]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),
    #[error("LettreAddress Error: {0}")]
    LettreAddressError(#[from] lettre::address::AddressError),
    #[error("Diesel Error: {0}")]
    DieselError(#[from] diesel::result::Error),
}