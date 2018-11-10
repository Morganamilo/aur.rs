//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`AurRequester`].
//!
//! [`AurRequester`]: trait.AurRequester.html

use constants::API_URI;
use model::{InfoResult, Search, SearchResult};
use std::fmt::{Display, Write};
use std::io::Read;
use reqwest::{Client as ReqwestClient, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json;
use {Error, Result};

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `reqwest` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use aur::AurReqwestRequester;
/// ```
///
/// At this point, the methods will be on your Reqwest Client.
pub trait AurRequester {
    /// Retrieves information about one or more packages along with metadata.
    ///
    /// # Examples
    ///
    /// Ensure that the `"rust-nightly"` package exists:
    ///
    /// ```rust
    /// extern crate aur;
    /// extern crate reqwest;
    ///
    /// use aur::bridge::reqwest::AurRequester;
    /// use reqwest::Client;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// let client = Client::new();
    ///
    /// let search = client.aur_info(&["rust-nightly"])?;
    ///
    /// assert_eq!(search.result_count, 1);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fmt`] if there was an error formatting the URI.
    ///
    /// Returns [`Error::Json`] if there was an error deserializing the
    /// response body.
    ///
    /// Returns [`Error::Reqwest`] if there was an error sending the request.
    ///
    /// Returns [`Error::ReqwestBad`] if the response status code was a 400.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the request was invalid.
    ///
    /// Returns [`Error::ReqwestParse`] if there was a parsing issue with the
    /// response.
    ///
    /// Returns [`Error::Uri`] if there was an error parsing the Uri.
    ///
    /// [`Error::Fmt`]: ../../enum.Error.html#variant.Fmt
    /// [`Error::Json`]: ../../enum.Error.html#variant.Json
    /// [`Error::Reqwest`]: ../../enum.Error.html#variant.Reqwest
    /// [`Error::ReqwestBad`]: ../../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../../enum.Error.html#variant.ReqwestParse
    /// [`Error::Uri`]: ../../enum.Error.html#variant.Uri
    fn aur_info<T: Display>(&self, packages: &[T])
        -> Result<Search<InfoResult>>;

    /// Searches for packages by a query, optionally filtering by maintainer
    /// name.
    ///
    /// # Examples
    ///
    /// Ensure that at least two packages return for the `"rust"` query, not
    /// specifying a maintainer:
    ///
    /// ```rust
    /// extern crate aur;
    /// extern crate reqwest;
    ///
    /// use aur::bridge::reqwest::AurRequester;
    /// use reqwest::Client;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// let client = Client::new();
    ///
    /// let search = client.aur_search(Some("rust-nightly"), None)?;
    ///
    /// assert!(search.result_count >= 2);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fmt`] if there was an error formatting the URI.
    ///
    /// Returns [`Error::Json`] if there was an error deserializing the
    /// response body.
    ///
    /// Returns [`Error::Reqwest`] if there was an error sending the request.
    ///
    /// Returns [`Error::ReqwestBad`] if the response status code was a 400.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the request was invalid.
    ///
    /// Returns [`Error::ReqwestParse`] if there was a parsing issue with the
    /// response.
    ///
    /// Returns [`Error::Uri`] if there was an error parsing the Uri.
    ///
    /// [`Error::Fmt`]: ../../enum.Error.html#variant.Fmt
    /// [`Error::Json`]: ../../enum.Error.html#variant.Json
    /// [`Error::Reqwest`]: ../../enum.Error.html#variant.Reqwest
    /// [`Error::ReqwestBad`]: ../../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../../enum.Error.html#variant.ReqwestParse
    /// [`Error::Uri`]: ../../enum.Error.html#variant.Uri
    fn aur_search(&self, query: Option<&str>, maintainer: Option<&str>)
        -> Result<Search<SearchResult>>;
}

impl AurRequester for ReqwestClient {
    fn aur_info<T: Display>(&self, packages: &[T])
        -> Result<Search<InfoResult>> {
        let mut url = format!("{}&type=info", API_URI);

        for package in packages {
            write!(url, "&arg[]={}", package)?;
        }

        let uri = Url::parse(&url)?;

        handle_request::<Search<InfoResult>>(self.get(uri))
    }

    fn aur_search(&self, query: Option<&str>, maintainer: Option<&str>)
        -> Result<Search<SearchResult>> {
        let mut url = format!("{}&type=search", API_URI);

        if let Some(query) = query {
            url.push_str("&arg=");
            url.push_str(query);
        }

        if let Some(maintainer) = maintainer {
            url.push_str("&maintainer=");
            url.push_str(maintainer);
        }

        let uri = Url::parse(&url)?;

        handle_request::<Search<SearchResult>>(self.get(uri))
    }
}

fn handle_request<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
    let response = request.send()?;

    match response.status() {
        StatusCode::OK => {},
        StatusCode::BAD_REQUEST => {
            return Err(Error::ReqwestBad(Box::new(response)));
        },
        _ => return Err(Error::ReqwestInvalid(Box::new(response))),
    }

    from_reader(response)
}

#[inline]
fn from_reader<T: DeserializeOwned, U: Read>(reader: U) -> Result<T> {
    serde_json::from_reader(reader).map_err(From::from)
}
