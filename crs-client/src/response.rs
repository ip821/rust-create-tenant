use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response<T> {
    #[serde(rename = "errorOrValue")]
    pub error_or_value: ErrorOrValue<T>,
}

#[derive(Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ErrorOrValue<T> {
    Error { error: Error },
    Value { value: T },
}

impl<T> ErrorOrValue<T> {
    pub fn unwrap(&self) -> &T {
        match self {
            ErrorOrValue::Error { error } => panic!("{}", error.message),
            ErrorOrValue::Value { value } => value
        }
    }
}