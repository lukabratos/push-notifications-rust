#[macro_use]
extern crate ureq;

use std::collections::HashMap;

pub fn publish(interests: Vec<String>, publish_request: HashMap<String, HashMap<String, HashMap<String, String>>>) {

    println!("{:?}", interests);
    println!("{:?}", publish_request);

    // Merge `interests` into the `publish_request`.

    let resp = ureq::post("https://1234.pushnotifications.pusher.com/publish_api/v1")
    .set("Accept", "application/json")
    .set("Content-Type", "application/json")
    .set("Authorization", "test-token")
    .send_json(json!(publish_request));

    if resp.ok() {
        println!("{:?}", resp);
    }
}
