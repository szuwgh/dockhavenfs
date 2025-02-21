use std::io;
use std::io::Error as IOError;
use thiserror::Error;

pub type IoResult<T> = Result<T, IoError>;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("Unexpected io: {0}")]
    UnexpectIO(io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("incompatible value")]
    ErrIncompatibleValue,
    #[error("volume already exists")]
    VolumeExists,
    #[error("disk access denied")]
    DiskAccessDenied,
}

impl From<&str> for IoError {
    fn from(e: &str) -> Self {
        IoError::Unexpected(e.to_string())
    }
}

impl From<String> for IoError {
    fn from(e: String) -> Self {
        IoError::Unexpected(e)
    }
}

impl From<IOError> for IoError {
    fn from(e: IOError) -> Self {
        IoError::UnexpectIO(e)
    }
}

impl From<IoError> for String {
    fn from(e: IoError) -> Self {
        format!("{}", e)
    }
}

pub type S3Result<T> = Result<T, IoError>;

#[derive(Error, Debug)]
pub enum S3Error {
    #[error("Unexpected io: {0}")]
    UnexpectIO(io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("incompatible value")]
    ErrIncompatibleValue,
    #[error("volume already exists")]
    VolumeExists,
    #[error("disk access denied")]
    DiskAccessDenied,
}

impl From<&str> for S3Error {
    fn from(e: &str) -> Self {
        S3Error::Unexpected(e.to_string())
    }
}

impl From<String> for S3Error {
    fn from(e: String) -> Self {
        S3Error::Unexpected(e)
    }
}

impl From<IOError> for S3Error {
    fn from(e: IOError) -> Self {
        S3Error::UnexpectIO(e)
    }
}

impl From<S3Error> for String {
    fn from(e: S3Error) -> Self {
        format!("{}", e)
    }
}
