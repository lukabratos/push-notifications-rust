#[macro_use]
extern crate ureq;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize)]
struct Payload {
    interests: Vec<String>,
    publish_request: serde_json::Value,
}

pub fn publish(
    instance_id: String,
    secret_key: String,
    interests: Vec<String>,
    publish_request: &str,
) {
    println!("{} {}", instance_id, secret_key);
    println!("{:?}", interests);
    println!("{:?}", publish_request);

    let publish_request = serde_json::from_str(publish_request).unwrap();

    let payload = Payload {
        interests: interests,
        publish_request: publish_request,
    };

    println!("{:?}", json!(payload));

    let resp = ureq::post(format!(
        "https://{}.pushnotifications.pusher.com/publish_api/v1/instances/{}/publishes",
        instance_id, instance_id
    )).set("Accept", "application/json")
        .set("Content-Type", "application/json")
        .set("Authorization", format!("Bearer {}", secret_key))
        .send_json(json!(payload));

    if resp.ok() {
        println!("{:?}", resp);
    } else {
        println!("{:?}", resp.status());
    }
}
