use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PSError<'a> {
    #[error("ssh io error")]
    IOError(#[from] io::Error),

    #[error("ssh connection error")]
    ConnectinError(&'a str),

    #[error("session error")]
    SessionError(&'a str),

    #[error("custom error")]
    CustomError(&'a str),
}
