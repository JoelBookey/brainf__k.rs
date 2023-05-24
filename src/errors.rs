#[derive(Debug)]
pub enum RuntimeError {
    UnexpectedToken,
    MissingBracket,
}

impl From<RuntimeError> for String {
    fn from(error: RuntimeError) -> Self {
        let error_msg = match error {
            RuntimeError::UnexpectedToken => "Unexpected Token",
            RuntimeError::MissingBracket => "No matching loop bracket",
        };
        String::from(error_msg)
    }
}
