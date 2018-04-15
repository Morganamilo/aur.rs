//! Bridge implementations for HTTP client libraries.

#[cfg(feature = "hyper")]
pub mod hyper;

#[cfg(feature = "reqwest")]
pub mod reqwest;
