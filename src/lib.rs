//! [![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs] [![rust badge]][rust link]
//!
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
//! # #[cfg(feature = "tokio")]
//! mod inner {
//! extern crate aur;
//! extern crate hyper;
//! extern crate hyper_tls;
//! extern crate tokio;
//!
//! #     fn main() -> Result<(), Box<::std::error::Error>> {
//! #
//! use aur::bridge::hyper::AurRequester;
//! use hyper::Client;
//! use hyper::net::HttpsConnector;
//! use hyper_tls::NativeTlsClient;
//!
//! let connector = HttpsConnector::new(4);
//! let client = Client::builder().build(connector);
//!
//! let done = client.aur_search(Some("rust-nightly"), None).map(|search| {
//!     assert!(search.result_count >= 2);
//! }).map_err(|why| {
//!     println!("Error getting rust-nightly info: {:?}", why);
//! });
//!
//! tokio::run(done);
//! #
//! #         Ok(())
//! #     }
//! # }
//! #
//! # #[cfg(feature = "tokio")]
//! # fn main() {
//! #     inner::main().unwrap();
//! # }
//! #
//! # #[cfg(not(feature = "tokio"))]
//! # fn main() {}
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
//! [ci]: https://travis-ci.org/zeyla/aur.rs
//! [ci-badge]: https://img.shields.io/travis/zeyla/aur.rs.svg?style=flat-square
//! [docs]: https://docs.rs/crate/aur
//! [docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg?style=flat-square
//! [LICENSE.md]: https://github.com/zeyla/aur.rs/blob/master/LICENSE.md
//! [license]: https://opensource.org/licenses/ISC
//! [license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [rust badge]: https://img.shields.io/badge/rust-1.29+-93450a.svg?style=flat-square
//! [rust link]: https://blog.rust-lang.org/2018/05/10/Rust-1.29.html
#![deny(missing_docs)]

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "http")]
extern crate http;
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
