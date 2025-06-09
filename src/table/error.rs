use bincode::error::{DecodeError, EncodeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("Table is full")]
    TableIsFull,
    #[error("Page is full")]
    PageIsFull,
    #[error("Failed to encode")]
    EncodeError(#[from] EncodeError),

    #[error("Failed to decode")]
    DecodeError(#[from] DecodeError),

    #[error("IO Error")]
    IoError(#[from] std::io::Error),
}
