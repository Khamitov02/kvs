use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    //IO error.
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Serde error")]
    Serde(#[from] serde_json::Error),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Unexpected command type")]
    UnexpectedCommandType,
}

//  impl From<io::Error> for KvsError {
//      fn from(err: io::Error) -> KvsError {
//          KvsError::Io(err)
//      }
//  }
pub type Result<T> = std::result::Result<T, KvsError>;
