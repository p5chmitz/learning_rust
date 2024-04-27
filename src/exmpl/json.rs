#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fmt::format, io::Write};

use json::{object, JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, ser::PrettyFormatter, Value};

// This example uses json which hasn't been updated since 2020
pub fn json_parsing() {
    let parsed: JsonValue = json::parse(
        r#"
        {
            "key": "12/23/1983",
            "anotherKey": "value",
            "object": {
                "nestedOne": "one",
                "nestedTwo": "two"
            }
        }
        "#,
    )
    .unwrap();
    let instantiated = object! {
        "key": "12/23/1983",
        "anotherKey": "value",
        "object": {
            "nestedOne": "one",
            "nestedTwo": "two"
        }
    };
    println!("The \"key's\" value is: \"{}\"", parsed["key"]);
    println!(
        "For the parsed object, the nested key \"nestedOne\" value is: \"{}\"",
        (parsed["object"]["nestedOne"])
    );
    println!(
        "For the instantiated object, the nested key \"nestedTwo\" value is: \"{}\"",
        (instantiated["object"]["nestedTwo"])
    );
}

// Custom type to serialize/deserialize
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: Option<String>,
    age: Option<u8>,
    hometown: Option<String>,
}

pub fn parsing_json_1() {
    // Deserializes a vector literal to a serde_json::Value enum type
    let json_vector_object: serde_json::Value = json!(["first", "second", "third"]);

    // Deserializes an object literal to a serde_json::Value enum type
    let json_object: serde_json::Value = json!({
        "first": "hello",
        "second": "world"
    });

    // Test vector literal with escape characters
    let json_vector_literal: String = String::from("[\"first\", \"second\", \"third\"]");
    // Deserialize JSON literal to serde_json::Value enum type
    let json_value: serde_json::Value = match serde_json::from_str(&json_vector_literal) {
        Ok(json_value) => json_value,
        Err(e) => {
            eprintln!("Error parsing JSON: {e}");
            return;
        }
    };
    // Deserializes a JSON string representing an array into a vector of string slices
    let vec: Vec<&str> = match serde_json::from_str(&json_vector_literal) {
        Ok(vec) => vec,
        Err(e) => {
            println!("Error parsing JSON");
            return;
        }
    };
    println!("Deserialized JSON object: {:?}", json_value);
    println!("Deserialized JSON vec: {:?}", vec);

    // Arbitrary type instance
    let person: Person = Person {
        name: Some("Peter".to_string()),
        age: Some(40),
        hometown: None,
    };
    // Serializes arbitrary type to String
    let person_json: String = match serde_json::to_string(&person) {
        Ok(json) => json,
        Err(e) => {
            println!("Error serializing struct");
            return;
        }
    };
    // Serializes arbitrary types to byte vector
    let person_bytes: Vec<u8> = match serde_json::to_vec(&person) {
        Ok(b) => b,
        Err(e) => {
            println!("Error serializing vector");
            return;
        }
    };
    println!("Serialized Person struct as string: \n{}", person_json);
    println!(
        "serialized Person struct as byte vector: \n{:?}",
        person_bytes
    );

    // Deserialize serde_json::Value to Person type
    //let person_value: serde_json::Value = json!({
    //    "name": "Peter",
    //    "age": 40
    //});
    let person_literal: &str = r#"{"name":"Peter","age":40}"#;
    let person_value: serde_json::Value = match serde_json::from_str(&person_literal) {
        Ok(value) => value,
        Err(e) => {
            println!("Error: line 121: {}", e);
            return;
        }
    };
    let person_data: Person = match serde_json::from_value(person_value) {
        Ok(d) => d,
        Err(e) => {
            println!("Error: line 128");
            return;
        }
    };
    println!("Deserialized Person struct: \n{:#?}", person_data);

    // Serializes a serde_json::Value enum type into a pretty-formatted JSON string
    let pretty_json: String = match serde_json::to_string_pretty(&json_value) {
        Ok(json_value) => json_value,
        Err(e) => {
            eprintln!("Error serializing JSON: {e}");
            return;
        }
    };

    // Deserializes literal to Value object
    let enrollment_value: serde_json::Value = json!({
        "type": "INDIVIDUAL",
        "registration": {
            "applicant": "Peter",
            "applicant_id": "123-45",
        }
    });
    let enrollment_string = serde_json::to_string(&enrollment_value).unwrap();
    println!("Serialized enrollment object: {}", &enrollment_string);
    let bytes = enrollment_string.clone().into_bytes();
    println!("Serialized enrollment object as bytes: {:?}", bytes);
}

/** Takes string args, creates a Value object, serializes it to a string, and then converts
 * the string to bytes */
fn enrollment_request_builder(name: &str, applicant_id: &str) -> Vec<u8> {
    // Creates serde_json::Value object
    let enrollment = json!({
        "type": "INDIVIDUAL",
        "registration": {
            "applicant": name,
            "applicant_id": applicant_id,
        }
    });

    // Serializes serde_json::Value object to a JSON string
    let json_string: String = match serde_json::to_string(&enrollment) {
        Ok(string) => string,
        Err(e) => {
            format!("Error deserializing object: {}", e)
        }
    };

    // Converts serialized JSON data to byte vector
    return json_string.into_bytes();
}

/** Takes raw JSON and pretty-formats it with specified ordering */
pub fn pretty_format_with_ordering(raw_json: &str) -> String {
    // Parse the raw JSON string into a serde_json::Value
    let parsed_json: serde_json::Value = serde_json::from_str(raw_json).unwrap_or_else(|e| {
        println!("Error deserializing raw JSON: {e}");
        serde_json::Value::Null
    });

    // Define the order of the output JSON object (does not have to be exhaustive)
    let keys_order = vec![
        "document",
        "title",
        "version",
        "type",
        "required",
        "properties",
    ];

    if let Some(parsed_map) = parsed_json.as_object() {
        // Creates the serialized data string to return
        let mut ordered_json_str = String::from("{\n");

        // Creates a vector to store all keys found in raw JSON
        let mut all_keys: Vec<String> = Vec::new();

        // Pushes keys found in the ordered list
        for key in &keys_order {
            if parsed_map.contains_key(*key) {
                all_keys.push(key.to_string());
            }
        }

        // Creates a HashSet of the keys for quick lookup
        let keys_order_set: std::collections::HashSet<&str> = keys_order.iter().cloned().collect();

        // Checks for keys not found in the user-defined list and
        // pushes any remaining keys ensuring all keys are serialized
        for key in parsed_map.keys() {
            if !keys_order_set.contains(key.as_str()) {
                all_keys.push(key.to_string());
            }
        }

        // Iterates through all key-value pairs and pushes serialized data to UTF-8 output string
        // This loop specifies three-space indenting in three places; if you change one, change all
        // of them
        for (i, key) in all_keys.iter().enumerate() {
            if let Some(value) = parsed_map.get(key) {
                let serialize_value = |value: &Value| -> String {
                    let mut buffer = Vec::new();
                    // Formats additive indentation for each level of nested object
                    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"   ");
                    let mut serliazer =
                        serde_json::ser::Serializer::with_formatter(&mut buffer, formatter);
                    value.serialize(&mut serliazer).unwrap_or_else(|e| {
                        println!("Error serializing value: {}", e);
                    });

                    String::from_utf8(buffer)
                        .unwrap_or_else(|e| format!("Error converting buffer to String: {}", e))
                };

                // Adds a comma before every key-value pair except the first one
                if i > 0 {
                    ordered_json_str.push_str(",\n");
                }

                // Adds indentation formatting for top-level key-value/object pairs;
                // The first gap is for top-level keys, the replace() gap is for objects
                ordered_json_str.push_str(&format!(
                    "   \"{}\': {}",
                    key,
                    serialize_value(value).replace("\n", "\n   ")
                ));
            }
        }

        ordered_json_str.push_str("\n}");
        return ordered_json_str;
    } else {
        String::from("Expected top-level JSON object")
    }
}
