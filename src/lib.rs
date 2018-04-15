//! # aur
//!
//! `aur` is a package for interacting with the [Arch User Repository] RPC API.
//!
//! It supports client trait implementations for both asynchronous `hyper` and
//! synchronous `reqwest`.
//!
//! ### Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! aur = "~0.1"
//! ```
//!
//! And the following to your `main.rs` or `lib.rs`:
//!
//! ```rust
//! extern crate aur;
//! ```
//!
//! There are two features: `hyper-support` and `reqwest-support`.
//! `hyper-support` is enabled by default. To enable `reqwest-support`, instead
//! depend on `aur` like so:
//!
//! ```toml
//! [dependencies.aur]
//! default-features = false
//! features = ["reqwest-support"]
//! version = "~0.1"
//! ```
//!
//! ### Examples
//!
//! Asynchronously request information for the `rust-nightly` package:
//!
//! ```rust
//! # #[cfg(feature = "tokio_core")]
//! extern crate aur;
//! # #[cfg(feature = "tokio_core")]
//! extern crate hyper;
//! # #[cfg(feature = "tokio_core")]
//! extern crate hyper_tls;
//! # #[cfg(feature = "tokio_core")]
//! extern crate tokio_core;
//!
//! # #[cfg(feature = "tokio_core")]
//! # fn try_main() -> Result<(), Box<::std::error::Error>> {
//! #
//! use aur::bridge::hyper::AurRequester;
//! use hyper::Client;
//! use hyper::net::HttpsConnector;
//! use hyper_tls::NativeTlsClient;
//! use tokio_core::Core;
//!
//! let core = Core::new();
//! let handle = core.handle();
//! let connector = HttpsConnector::new(4, &handle)?;
//! let client = Client::configure().connector(connector).build(&handle);
//!
//! let done = client.aur_search(Some("rust-nightly"), None).map(|search| {
//!     assert!(search.result_count >= 2);
//! }).map_err(|_| ());
//!
//! core.run(done)?;
//! #     Ok(())
//! # }
//! #
//! # #[cfg(not(feature = "tokio_core"))]
//! # fn try_main() -> Result<(), ()> { Ok(()) }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! Synchronously request information for the `rust-nightly` package:
//!
//! ```rust
//! # #[cfg(feature = "reqwest")]
//! extern crate aur;
//! # #[cfg(feature = "reqwest")]
//! extern crate reqwest;
//!
//! # #[cfg(feature = "reqwest")]
//! # fn try_main() -> Result<(), Box<::std::error::Error>> {
//! #
//! use aur::bridge::reqwest::AurRequester;
//! use reqwest::Client;
//!
//! let client = Client::new();
//!
//! let info = client.aur_info(&["rust-nightly"])?;
//!
//! match info.results.first() {
//!     Some(package) => {
//!         if let Some(ref maintainer) = package.maintainer {
//!             println!("The package is maintained by: {}", maintainer);
//!         } else {
//!             println!("The package has no maintainer");
//!         }
//!     },
//!     None => {
//!         println!("No package found!");
//!     },
//! }
//! #     Ok(())
//! # }
//! #
//! # #[cfg(not(feature = "reqwest"))]
//! # fn try_main() -> Result<(), Box<::std::error::Error>> { Ok(()) }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! ### License
//!
//! ISC.
//!
//! [Arch User Repository]: https://aur.archlinux.org/
#![deny(missing_docs)]

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "reqwest")]
extern crate reqwest;

pub mod bridge;
pub mod model;

mod constants;
mod error;

pub use error::{Error, Result};

#[cfg(feature = "hyper")]
pub use self::bridge::hyper::AurRequester as AurHyperRequester;
#[cfg(feature = "reqwest")]
pub use self::bridge::reqwest::AurRequester as AurReqwestRequester;
