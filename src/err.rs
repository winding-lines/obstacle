use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ObstacleError {
    pub message: String,
}

impl Display for ObstacleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObstacleError: {:#?}", self.message)
    }
}

impl ObstacleError {
    pub fn new<S: Into<String>>(msg: S) -> Self {
        ObstacleError {
            message: msg.into(),
        }
    }

    pub fn from_err<S: Error>(err: S) -> Self {
        ObstacleError {
            message: err.to_string(),
        }
    }
}

pub fn obstinate_err<T, S: AsRef<str>>(msg: S) -> Result<T, ObstacleError> {
    Err(ObstacleError::new(msg.as_ref()))
}

impl From<std::io::Error> for ObstacleError {
    fn from(err: std::io::Error) -> Self {
        ObstacleError::from_err(err)
    }
}

impl From<regex::Error> for ObstacleError {
    fn from(err: regex::Error) -> Self {
        ObstacleError::from_err(err)
    }
}

impl From<object_store::Error> for ObstacleError {
    fn from(err: object_store::Error) -> Self {
        ObstacleError::from_err(err)
    }
}

impl From<url::ParseError> for ObstacleError {
    fn from(err: url::ParseError) -> Self {
        ObstacleError::from_err(err)
    }
}

impl From<object_store::path::Error> for ObstacleError {
    fn from(err: object_store::path::Error) -> Self {
        ObstacleError::from_err(err)
    }
}
