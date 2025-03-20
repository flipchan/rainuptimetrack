//! Error lib
use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Invalid network selected
    InvalidNetwork,
    /// invalid ethereum address
    InvalidAddressFormat,
    /// Serde Json error
    SerdeJson(serde_json::error::Error),
    InvalidUrl(String),
    RecvError(String),
    /// could not parse rpc url
    UrlParseError(url::ParseError),
    /// problems caused by hypersync_client
    //   HyperSyncError(hypersync_client::Error),
    Io(String),
    FileIOerror(std::io::Error),
    // std::error::Error as a string
    Stderror(String),
    /// could not find field
    MissingField(String),
    /// Could not convert data
    Conversion(String),
    /// problem parsing CSV
    CSVError(csv::Error),
    // error in std::error
    STDLIBerror,
    /// could not build hypersync query
    QueryError,
    AnyhowError(anyhow::Error),
    ConnectionSubscriptionProblem,
    Client(Box<dyn core::error::Error + Send + Sync + 'static>),
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Self::CSVError(value)
    }
}

// rpc url parsing
impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}

// hypersync demands anyhow so lets add error support for it
impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::AnyhowError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::FileIOerror(value)
    }
}

// support std::error::Error
impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::Stderror(error.to_string())
    }
}
impl From<&dyn std::error::Error> for Error {
    fn from(error: &dyn std::error::Error) -> Self {
        Self::Stderror(format!("{error:?}"))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SerdeJson(e) => Some(e),
            Self::FileIOerror(e) => Some(e),
            Self::Client(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

// Add proper Display implementation
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNetwork => write!(f, "Invalid network selected"),
            Self::InvalidAddressFormat => write!(f, "Invalid Ethereum address format"),
            /* Add display implementations for all variants */
            _ => write!(f, "Unknown error occurred"),
        }
    }
}
