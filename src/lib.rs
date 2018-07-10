#[macro_use]
extern crate ureq;

use std::collections::HashMap;

pub fn publish(instance_id: String, secret_key: String, interests: Vec<String>, publish_request: HashMap<String, HashMap<String, HashMap<String, String>>>) {

    println!("{} {}", instance_id, secret_key);
    println!("{:?}", interests);
    println!("{:?}", publish_request);

    // Merge `interests` into the `publish_request`.

    let resp = ureq::post(format!("https://{}.pushnotifications.pusher.com/publish_api/v1", instance_id))
    .set("Accept", "application/json")
    .set("Content-Type", "application/json")
    .set("Authorization", format!("Bearer {}", secret_key))
    .send_json(json!(publish_request));

    if resp.ok() {
        println!("{:?}", resp);
    }
}
