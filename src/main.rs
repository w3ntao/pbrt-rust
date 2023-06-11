mod cameras;
mod film;
mod filter;
mod math;
mod pbrt;
mod scene_parser;

use crate::pbrt::*;

fn json_hello_world() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let value: Value = serde_json::from_str(data).unwrap();
    let phones = &value["phones"];

    //println!("phone[0]: {}", phones[0]);
    //println!("phone[3]: {}", phones[3]);

    let phones_array = phones.as_array().unwrap();
    for x in phones_array {
        println!("phone: {}", x);
    }
}

fn main() {
    //json_hello_world();

    let mut builder = SceneBuilder::new("killeroo-floor.json");
    builder.build_scene();
}
