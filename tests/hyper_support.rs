extern crate aur;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use aur::{
    model::SearchBy,
    AurHyperRequester,
};
use futures::Future;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;

#[inline]
fn client() -> Client<HttpsConnector<HttpConnector>, Body> {
	Client::builder().build(HttpsConnector::new(4).unwrap())
}

#[test]
fn test_info() {
	let done = client().aur_info(&["rust-nightly"]).map(|search| {
        assert_eq!(search.result_count, 1);
    }).map_err(|why| {
        panic!("Err testing info: {:?}", why);
    });

	tokio::run(done);
}

#[test]
fn test_orphans() {
	let done = client().aur_orphans().map(|search| {
        assert!(search.result_count >= 2);
    }).map_err(|why| {
        panic!("Err searching: {:?}", why);
    });

	tokio::run(done);
}

#[test]
fn test_search() {
	let done = client().aur_search("rust").map(|search| {
        assert!(search.result_count >= 2);
    }).map_err(|why| {
        panic!("Err searching: {:?}", why);
    });

	tokio::run(done);
}

#[test]
fn test_search_by() {
	let done = client().aur_search_by("rust", SearchBy::Name).map(|search| {
        assert!(search.result_count >= 2);
    }).map_err(|why| {
        panic!("Err searching: {:?}", why);
    });

	tokio::run(done);
}
