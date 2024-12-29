#[derive(Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    ValidationError(Vec<String>),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(err)
            | ApiError::InternalServerError(err)
            | ApiError::InvalidData(err)
            | ApiError::Conflict(err)
            | ApiError::NotFound(err)
            | ApiError::Unknown(err) => writeln!(f, "{}", err),
            ApiError::ValidationError(mex_vec) => mex_vec.iter().fold(Ok(()), |result, err| {
                result.and_then(|_| writeln!(f, "{},", err))
            }),
        }
    }
}
