use std::fmt;

pub enum Error {
    AlgoAPI,
    BadUrl(String),
}

pub type AppResult<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AlgoAPI => write!(f, "error calling Algorithmia."),
            Error::BadUrl(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
