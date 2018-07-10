#[macro_use]
extern crate ureq;

use std::collections::HashMap;

pub fn publish(interests: Vec<String>, publish_request: HashMap<String, HashMap<String, HashMap<String, String>>>) {

    println!("{:?}", interests);
    println!("{:?}", publish_request);

    let resp = ureq::post("http://httpbin.org/post")
    .set("X-Luka-Header", "Test")
    .send_json(json!({
        "a": "aaaa",
        "b": "bbbb"
    }));

    if resp.ok() {
        println!("{:?}", resp);
    }
}
