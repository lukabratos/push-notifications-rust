#[macro_use]
extern crate ureq;

pub fn publish() {
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
