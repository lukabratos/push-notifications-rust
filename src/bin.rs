extern crate beams;

use beams::publish;

use std::collections::HashMap;

pub fn main() {
    let interests = vec![String::from("pizza"), String::from("avocado")];

    let mut alert = HashMap::new();
    alert.insert(String::from("alert"), String::from("hi"));

    let mut aps = HashMap::new();
    aps.insert(String::from("aps"), alert);
    
    let mut publish_request = HashMap::new();
    publish_request.insert(String::from("apns"), aps);

    publish(interests, publish_request);
}
