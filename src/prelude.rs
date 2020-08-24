pub enum EmailError {
    InternalError(String),
    WrongEmailAddress(String),
}

impl EmailError {
    pub fn internal_error(msg: &str) -> Self {
        EmailError::InternalError(msg.to_string())
    }
    pub fn wrong_email_address(msg: &str) -> Self {
        EmailError::WrongEmailAddress(msg.to_string())
    }
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::InternalError(msg) => write!(f, "Email küldési hiba. {}", msg),
            EmailError::WrongEmailAddress(msg) => write!(f, "Emailcím hiba. {}", msg),
        }
    }
}

impl std::fmt::Debug for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&"EmailError".to_string())
            .field(self)
            .finish()
    }
}

impl From<EmailError> for ::tonic::Status {
    fn from(error: EmailError) -> Self {
        match error {
            EmailError::InternalError(msg) => ::tonic::Status::internal(msg),
            EmailError::WrongEmailAddress(msg) => {
                ::tonic::Status::invalid_argument(format!("Hibás email cím: {}", msg))
            }
        }
    }
}

impl From<::storaget::PackError> for EmailError {
    fn from(error: ::storaget::PackError) -> Self {
        match error {
            _ => EmailError::internal_error(&format!("Storage error! {}", error)),
        }
    }
}

pub type EmailResult<T> = Result<T, EmailError>;
