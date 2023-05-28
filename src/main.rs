mod square_matrix;
mod scene_builder;

use std::fs::File;
use std::io::Read;

use serde_json::Value;
use crate::scene_builder::SceneBuilder;
use crate::square_matrix::SquareMatrix;

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
    let builder = SceneBuilder::build_from_json("data.json");

    let matrix = SquareMatrix::<3>::default();
}
