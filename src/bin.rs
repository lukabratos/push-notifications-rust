extern crate beams;

use beams::publish;

pub fn main() {
    let interests = vec![String::from("pizza"), String::from("avocado")];

    let publish_request = r#"{
        "apns": {
            "aps": {
                "alert": "Hello!"
            }
        },
        "fcm": {
            "notification": {
                "title": "Hello!",
                "body": "Hello, world!"
            }
        }
    }"#;

    publish(
        String::from("id"),
        String::from("key"),
        interests,
        publish_request,
    );
}
