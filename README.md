[![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs] [![rust badge]][rust link]

# aur

`aur` is a package for interacting with the [Arch User Repository] RPC API.

It supports client trait implementations for both asynchronous `hyper` and
synchronous `reqwest`.

### Installation

This library requires at least Rust 1.21.0.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aur = "~0.1"
```

And the following to your `main.rs` or `lib.rs`:

```rust
extern crate aur;
```

There are two features: `hyper-support` and `reqwest-support`.
`hyper-support` is enabled by default. To enable `reqwest-support`, instead
depend on `aur` like so:

```toml
[dependencies.aur]
default-features = false
features = ["reqwest-support"]
version = "~0.1"
```

### Examples

Asynchronously request information for the `rust-nightly` package:

```rust
extern crate aur;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use aur::bridge::hyper::AurRequester;
use hyper::Client;
use tokio_core::HttpsConnector;
use tokio_core::Core;

let core = Core::new()?;
let handle = core.handle();
let connector = HttpsConnector::new(4, &handle)?;
let client = Client::configure().connector(connector).build(&handle);

let done = client.aur_search(Some("rust-nightly"), None).map(|search| {
    assert!(search.result_count >= 2);
}).map_err(|_| ());
```

Synchronously request information for the `rust-nightly` package:

```rust
extern crate aur;
extern crate reqwest;

use aur::bridge::reqwest::AurRequester;
use reqwest::Client;

let client = Client::new();

let info = client.aur_info(&["rust-nightly"])?;

match info.first() {
    Some(package) => {
        if let Some(ref maintainer) = package.maintainer {
            println!("The package is maintained by: {}", maintainer);
        } else {
            println!("The package has no maintainer");
        }
    },
    None => {
        println!("No package found!");
    },
}
```

### License

[ISC][LICENSE.md].

[ci]: https://travis-ci.org/zeyla/aur.rs
[ci-badge]: https://img.shields.io/travis/zeyla/aur.rs.svg?style=flat-square
[docs]: https://docs.rs/crate/aur
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg?style=flat-square
[LICENSE.md]: https://github.com/zeyla/aur.rs/blob/master/LICENSE.md
[license]: https://opensource.org/licenses/ISC
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[rust badge]: https://img.shields.io/badge/rust-1.21+-93450a.svg?style=flat-square
[rust link]: https://blog.rust-lang.org/2017/10/12/Rust-1.21.html
