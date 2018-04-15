use serde_json::Error as JsonError;
use std::{
    error::Error as StdError,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

#[cfg(feature = "hyper")]
use hyper::error::{Error as HyperError, UriError};
#[cfg(feature = "reqwest")]
use reqwest::{
    Error as ReqwestError,
    Response as ReqwestResponse,
    UrlError as ReqwestUrlError,
};

/// Standard result type for asynchronous functions throughout the library.
pub type Result<T> = StdResult<T, Error>;

/// Enum encompassing the library's possible returned errors.
#[derive(Debug)]
pub enum Error {
    /// An error that occurred while formatting a string.
    Fmt(FmtError),
    /// An error from the `serde_json` crate while deserializing the body of an
    /// HTTP response.
    Json(JsonError),
    /// An error from the `hyper` crate while performing an HTTP request.
    #[cfg(feature = "hyper")]
    Hyper(HyperError),
    /// An error from the `reqwest` crate while performing an HTTP request.
    #[cfg(feature = "reqwest")]
    Reqwest(ReqwestError),
    /// An error indicating a bad request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestBad(Box<ReqwestResponse>),
    /// An error indicating an invalid request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestInvalid(Box<ReqwestResponse>),
    /// An error indicating a parsing issue when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestParse(ReqwestUrlError),
    /// An error when building a request's URI from the `hyper` crate when it is
    /// enabled.
    #[cfg(feature = "hyper")]
    Uri(UriError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Fmt(ref inner) => inner.description(),
            #[cfg(feature = "hyper")]
            Error::Hyper(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            #[cfg(feature = "reqwest")]
            Error::Reqwest(ref inner) => inner.description(),
            #[cfg(feature = "reqwest")]
            Error::ReqwestBad(_) => "Request bad",
            #[cfg(feature = "reqwest")]
            Error::ReqwestInvalid(_) => "Request invalid",
            #[cfg(feature = "reqwest")]
            Error::ReqwestParse(ref inner) => inner.description(),
            #[cfg(feature = "hyper")]
            Error::Uri(ref inner) => inner.description(),
        }
    }
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Self {
        Error::Fmt(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

#[cfg(feature = "hyper")]
impl From<HyperError> for Error {
    fn from(err: HyperError) -> Self {
        Error::Hyper(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestUrlError> for Error {
    fn from(err: ReqwestUrlError) -> Self {
        Error::ReqwestParse(err)
    }
}

#[cfg(feature = "hyper")]
impl From<UriError> for Error {
    fn from(err: UriError) -> Error {
        Error::Uri(err)
    }
}
