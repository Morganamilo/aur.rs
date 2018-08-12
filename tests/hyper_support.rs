extern crate aur;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use aur::AurHyperRequester;
use futures::Future;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

#[inline]
fn client() -> Client<HttpsConnector<HttpConnector>, Body> {
	Client::builder().build(HttpsConnector::new(4).unwrap())
}

#[test]
fn test_info() {
	let mut core = Core::new().unwrap();
	let client = client();

	let done = client.aur_info(&["rust-nightly"]).map(|search| {
        assert_eq!(search.result_count, 1);
    }).map_err(|_| ());

	core.run(done).expect("core err");
}

#[test]
fn test_search() {
	let mut core = Core::new().unwrap();
	let client = client();

	let done = client.aur_search(Some("rust"), None).map(|search| {
        assert!(search.result_count >= 2);
    }).map_err(|why| {
        panic!("Err searching: {:?}", why);
    });

	core.run(done).expect("core err");
}
