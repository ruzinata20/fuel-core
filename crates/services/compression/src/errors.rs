use fuel_core_storage::Error as StorageError;
use thiserror::Error;

/// Errors that can occur during compression
#[derive(Error, Debug)]
pub enum CompressionError {
    // L2 block source Errors
    /// Failed to get l2 block
    #[error("failed to get l2 block: `{0}`")]
    FailedToGetBlock(String),
    // Compression storage Errors
    /// Failed to read compressed block from storage
    #[error("failed to write compressed block to storage: `{0}`")]
    FailedToWriteCompressedBlock(StorageError),
    /// Failed to commit storage transaction
    #[error("failed to commit transaction: `{0}`")]
    FailedToCommitTransaction(StorageError),
    /// Failed to get latest height from storage
    #[error("failed to get latest height from storage: `{0}`")]
    FailedToGetLatestHeight(StorageError),
    // Configuration Errors
    /// Failed to get config
    #[error("failed to get configuration: `{0}`")]
    FailedToGetConfig(String),
    // Service errors
    /// Failed to create service
    #[error("failed to create service: `{0}`")]
    FailedToCreateService(String),
    /// Failed to compress block
    #[error("failed to compress block: `{0}`")]
    FailedToCompressBlock(anyhow::Error),
    /// Failed to handle new block
    #[error("failed to handle new block: `{0}`")]
    FailedToHandleNewBlock(String),
}
