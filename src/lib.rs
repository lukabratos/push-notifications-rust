extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

pub fn publish() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let url : Uri = "http://httpbin.org/response-headers?foo=bar".parse().unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    let request = client.get(url)
        .map(|res| {
            println!("{}", res.status());
            assert_eq!(res.status(), hyper::Ok);
        });

    core.run(request).unwrap();
}
