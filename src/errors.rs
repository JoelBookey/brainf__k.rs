#[derive(Debug)]
pub enum RuntimeError {
    UnexpectedToken,
}

impl From<RuntimeError> for String {
    fn from(error: RuntimeError) -> Self {
        let error_msg = match error {
            RuntimeError::UnexpectedToken => "Unexpected Token",
        };
        String::from(error_msg)
    }
}
