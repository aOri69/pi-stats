#[derive(Debug)]
pub enum Error {
    ThrottleRead,
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    CtrlcHandler(ctrlc::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ThrottleRead => write!(f, "Throttle parsing error"),
            Error::Io(e) => write!(f, "IO error: {e}"),
            Error::ParseInt(e) => write!(f, "ParseInt error: {e}"),
            Error::CtrlcHandler(e) => write!(f, "Ctrlc handler error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ThrottleRead => None,
            Error::Io(e) => Some(e),
            Error::ParseInt(e) => Some(e),
            Error::CtrlcHandler(e) => Some(e),
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

impl From<ctrlc::Error> for Error {
    fn from(e: ctrlc::Error) -> Self {
        Error::CtrlcHandler(e)
    }
}
