#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ParseCommand(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {e}"),
            Error::ParseInt(e) => write!(f, "ParseInt error: {e}"),
            Error::Utf8(e) => write!(f, "Command output UTF8 exception: {e}"),
            Error::ParseCommand(e) => write!(f, "Command output parsing exception: {e}"),
            Error::ParseFloat(e) => write!(f, "ParseFloat error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::ParseInt(e) => Some(e),
            Error::Utf8(e) => Some(e),
            Error::ParseCommand(_) => None,
            Error::ParseFloat(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::Utf8(e)
    }
}
