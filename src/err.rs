use std::error::Error;

#[derive(Debug)]
pub struct ObstinateError {
    pub message: String,
}

impl ObstinateError {
    pub fn new<S: Into<String>>(msg: S) -> Self {
        ObstinateError {
            message: msg.into(),
        }
    }

    pub fn from_err<S: Error>(err: S) -> Self {
        ObstinateError {
            message: err.to_string(),
        }
    }
}

pub fn obstinate_err<T, S: AsRef<str>>(msg: S) -> Result<T, ObstinateError> {
    Err(ObstinateError::new(msg.as_ref()))
}

impl From<std::io::Error> for ObstinateError {
    fn from(err: std::io::Error) -> Self {
        ObstinateError::from_err(err)
    }
}

impl From<regex::Error> for ObstinateError {
    fn from(err: regex::Error) -> Self {
        ObstinateError::from_err(err)
    }
}

impl From<object_store::Error> for ObstinateError {
    fn from(err: object_store::Error) -> Self {
        ObstinateError::from_err(err)
    }
}

impl From<url::ParseError> for ObstinateError {
    fn from(err: url::ParseError) -> Self {
        ObstinateError::from_err(err)
    }
}
