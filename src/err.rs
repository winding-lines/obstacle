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
