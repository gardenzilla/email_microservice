pub enum EmailServiceError {
    InternalError(String),
    WrongEmail(String),
}

impl EmailServiceError {
    pub fn internal_error(msg: &str) -> Self {
        EmailServiceError::InternalError(msg.to_string())
    }
    pub fn wrong_email(msg: &str) -> Self {
        EmailServiceError::WrongEmail(msg.to_string())
    }
}

impl std::fmt::Display for EmailServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailServiceError::InternalError(msg) => write!(f, "{}", msg),
            EmailServiceError::WrongEmail(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::fmt::Debug for EmailServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&"EmailServiceError".to_string())
            .field(self)
            .finish()
    }
}

impl From<EmailServiceError> for ::tonic::Status {
    fn from(error: EmailServiceError) -> Self {
        match error {
            EmailServiceError::InternalError(msg) => ::tonic::Status::internal(msg),
            EmailServiceError::WrongEmail(msg) => ::tonic::Status::invalid_argument(msg),
        }
    }
}

impl From<::storaget::PackError> for EmailServiceError {
    fn from(error: ::storaget::PackError) -> Self {
        match error {
            _ => EmailServiceError::internal_error(&error.to_string()),
        }
    }
}

pub type ServiceResult<T> = Result<T, EmailServiceError>;
