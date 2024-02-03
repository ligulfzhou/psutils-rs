use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PSError {
    #[error("ssh io error")]
    IOError(#[from] io::Error),

    #[error("ssh connection error: {:}", .0)]
    ConnectinError(String),

    #[error("session error: {:}", .0)]
    SessionError(String),

    #[error("custom error: {:}", .0)]
    CustomError(String),
}

pub type PSResult<T> = Result<T, PSError>;
