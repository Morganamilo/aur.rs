//! Bridge to provide a client implementation for the `hyper` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`AurRequester`].
//!
//! [`AurRequester`]: trait.AurRequester.html

use constants::API_URI;
use futures::{Future, Stream, future};
use hyper::body::Body;
use hyper::client::connect::Connect;
use hyper::client::Client as HyperClient;
use hyper::{Request, Uri};
use model::{InfoResult, Search, SearchResult};
use serde_json;
use std::fmt::{Display, Write};
use std::str::FromStr;
use Error;

macro_rules! ftry {
    ($code:expr) => {
        match $code {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(From::from(why))),
        }
    }
}

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `hyper` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use aur::AurHyperRequester;
/// ```
///
/// At this point, the methods will be on your Hyper Client.
pub trait AurRequester {
    /// Retrieves information about one or more packages along with metadata.
    ///
    /// # Examples
    ///
    /// Ensure that the `"rust-nightly"` package exists:
    ///
    /// ```rust,ignore
    /// extern crate aur;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate tokio_core;
    ///
    /// use aur::bridge::hyper::AurRequester;
    /// use hyper::Client;
    /// use hyper_tls::HttpsConnector;
    /// use tokio_core::Core;
    ///
    /// let core = Core::new()?;
    ///
    /// let handle = core.handle();
    /// let connector = HttpsConnector::new(4, &handle)?;
    /// let client = Client::configure().connector(connector).build(&handle);
    ///
    /// let done = client.aur_info(&["rust-nightly"]).map(|search| {
    ///     assert_eq!(search.result_count, 1);
    /// }).map_err(|_| ());
    /// ```
    ///
    /// # Errors
    ///
    /// Resolves to [`Error::Fmt`] if there was an error formatting the URI.
    ///
    /// Resolves to [`Error::Hyper`] if there was an error sending the request.
    ///
    /// Resolves to [`Error::Json`] if there was an error deserializing the
    /// response body.
    ///
    /// Resolves to [`Error::Uri`] if there was an error parsing the Uri.
    ///
    /// [`Error::Fmt`]: ../../enum.Error.html#variant.Fmt
    /// [`Error::Hyper`]: ../../enum.Error.html#variant.Hyper
    /// [`Error::Json`]: ../../enum.Error.html#variant.Json
    /// [`Error::Uri`]: ../../enum.Error.html#variant.Uri
    fn aur_info<T: Display>(&self, packages: &[T])
        -> Box<Future<Item = Search<InfoResult>, Error = Error>>;

    /// Searches for packages by a query, optionally filtering by maintainer
    /// name.
    ///
    /// # Examples
    ///
    /// Ensure that at least two packages return for the `"rust"` query, not
    /// specifying a maintainer:
    ///
    /// ```rust,ignore
    /// extern crate aur;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate tokio_core;
    ///
    /// use aur::bridge::hyper::AurRequester;
    /// use hyper::Client;
    /// use hyper_tls::HttpsConnector;
    /// use tokio_core::Core;
    ///
    /// let handle = core.handle();
    /// let connector = HttpsConnector::new(4, handle)?;
    /// let client = Client::configure().connector(connector).build(&handle);
    ///
    /// let done = client.aur_search(Some("rust"), None).map(|search| {
    ///     assert!(search.result_count >= 2);
    /// }).map_err(|_| ());
    /// ```
    ///
    /// # Errors
    ///
    /// Resolves to [`Error::Fmt`] if there was an error formatting the URI.
    ///
    /// Resolves to [`Error::Hyper`] if there was an error sending the request.
    ///
    /// Resolves to [`Error::Json`] if there was an error deserializing the
    /// response body.
    ///
    /// Resolves to [`Error::Uri`] if there was an error parsing the Uri.
    ///
    /// [`Error::Fmt`]: ../../enum.Error.html#variant.Fmt
    /// [`Error::Hyper`]: ../../enum.Error.html#variant.Hyper
    /// [`Error::Json`]: ../../enum.Error.html#variant.Json
    /// [`Error::Uri`]: ../../enum.Error.html#variant.Uri
    fn aur_search(&self, query: Option<&str>, maintainer: Option<&str>)
        -> Box<Future<Item = Search<SearchResult>, Error = Error>>;
}

impl<C> AurRequester for HyperClient<C, Body>
    where C: Connect + Sync + 'static,
          C::Future: 'static,
          C::Transport: 'static {
    fn aur_info<T: Display>(&self, packages: &[T])
        -> Box<Future<Item = Search<InfoResult>, Error = Error>> {
        let mut url = format!("{}&type=info", API_URI);

        for package in packages {
            if let Err(why) = write!(url, "&arg[]={}", package) {
                return Box::new(future::err(Error::Fmt(why)));
            }
        }

        let c = &url;
        let uri = ftry!(Uri::from_str(c));

        let mut request = Request::get(uri);
        let req = ftry!(request.body(Body::empty()));

        Box::new(self.request(req)
            .and_then(|res| res.into_body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn aur_search(&self, query: Option<&str>, maintainer: Option<&str>)
        -> Box<Future<Item = Search<SearchResult>, Error = Error>> {
        let mut url = format!("{}&type=search", API_URI);

        if let Some(query) = query {
            url.push_str("&arg=");
            url.push_str(query);
        }

        if let Some(maintainer) = maintainer {
            url.push_str("&maintainer=");
            url.push_str(maintainer);
        }

        let c = &url;
        let uri = ftry!(Uri::from_str(c));

        let mut request = Request::get(uri);
        let req = ftry!(request.body(Body::empty()));

        Box::new(self.request(req)
            .and_then(|res| res.into_body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }
}
