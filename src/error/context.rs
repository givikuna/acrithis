#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContext {
    pub message : String,
    pub cause :   Option<String>,
}

impl ErrorContext {
    pub fn new<M : Into<String>>(message : M) -> Self {
        Self { message : message.into(),
               cause :   None, }
    }

    pub fn with_cause<C : std::fmt::Display>(mut self, cause : C) -> Self {
        self.cause = Some(cause.to_string());
        self
    }
}
