use bincode::error::EncodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("Table is full")]
    TableIsFull,
    #[error("Page is full")]
    PageIsFull,
    #[error("Failed to encode")]
    EncodeError(#[from] EncodeError),
}
