use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogicError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serde JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Invalid battle checkpoint: {0}")]
    InvalidBattleCheckpoint(String),

    #[error("Hero not found")]
    HeroNotFound,

    #[error("Insufficient items")]
    InsufficientItems,

    #[error("Insufficient funds")]
    InsufficientCurrency,

    #[error("Banner not found")]
    BannerNotFound,

    #[error("Banner is not yet active")]
    BannerNotYetActive,

    #[error("Banner has expired")]
    BannerExpired,
}

impl From<anyhow::Error> for LogicError {
    fn from(error: anyhow::Error) -> Self {
        Self::Custom(error.to_string())
    }
}

pub(crate) use LogicError as AppError;
