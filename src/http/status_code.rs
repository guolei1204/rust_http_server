use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}
impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "Ok",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "NotFound",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}
