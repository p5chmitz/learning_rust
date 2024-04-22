#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::Write;

//use json::{object, JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, ser::PrettyFormatter, Result};

//JSON parsing experiment
// This example uses json which hasn't been updated since 2020
//pub fn json_parsing() {
//    let parsed: JsonValue = json::parse(
//        r#"
//        {
//            "key": "12/23/1983",
//            "anotherKey": "value",
//            "object": {
//                "nestedOne": "one",
//                "nestedTwo": "two"
//            }
//        }
//        "#,
//    )
//    .unwrap();
//    let instantiated = object! {
//        "key": "12/23/1983",
//        "anotherKey": "value",
//        "object": {
//            "nestedOne": "one",
//            "nestedTwo": "two"
//        }
//    };
//    println!("The \"key's\" value is: \"{}\"", parsed["key"]);
//    println!(
//        "For the parsed object, the nested key \"nestedOne\" value is: \"{}\"",
//        (parsed["object"]["nestedOne"])
//    );
//    println!(
//        "For the instantiated object, the nested key \"nestedTwo\" value is: \"{}\"",
//        (instantiated["object"]["nestedTwo"])
//    );
//}

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

/** Takes unordered Value object serializes it to a specified order */
pub fn pretty_format_with_ordering(raw_json: &str) -> String {
    // Creates serde_json::Value object for manipulation (a tale in three acts)
    //
    // This requires an "empty" return value; kinda gross
    //let parsed_json = match serde_json::from_str(raw_json) {
    //    Ok(obj) => obj,
    //    Err(e) => {
    //        println!("Error creating JSON object: {}", e);
    //        serde_json::Value::Null
    //    }
    //};
    //// The infamous turbofish type annotation is required; what a mess!
    //if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(raw_json) {
    //} else {
    //    println!("Error deserializing JSON string");
    //}
    // Doesn't panic, is compact, but still requires a null object expression
    let parsed_json: serde_json::Value = serde_json::from_str(raw_json).unwrap_or_else(|e| {
        println!("Error deserializing JSON: {e}");
        serde_json::Value::Null
    });

    // Creates ordered key vector
    let keys_order = vec![
        "document",
        "title",
        "version",
        "type",
        "required",
        "properties",
    ];

    if let serde_json::Value::Object(parsed_map) = parsed_json {
        // IDK?
        let mut ordered_json_str = String::from("{\n");

        // Creates a set of ordering keys for quick lookup
        let keys_order_set: std::collections::HashSet<&str> = keys_order.iter().cloned().collect();

        // Filter and collect specified keys in parsed_map
        let mut all_keys: Vec<&String> = parsed_map
            .keys()
            .filter(|k| keys_order_set.contains(k.as_str()))
            .collect();

        // Sort the filtered keys according to their order in keys_order
        all_keys.sort_by_key(|k| keys_order.iter().position(|&order| order == k.as_str()));

        // Append remaining keys not specified in keys_order array
        all_keys.extend(
            parsed_map
                .keys()
                .filter(|k| !keys_order_set.contains(k.as_str())),
        );

        for (i, key) in all_keys.iter().enumerate() {
            if let Some(value) = parsed_map.get(*key) {
                let mut buffer = Vec::new();
                let formatter = serde_json::ser::PrettyFormatter::with_indent(b"   ");
                let mut serliazer =
                    serde_json::ser::Serializer::with_formatter(&mut buffer, formatter);
                value
                    .serialize(&mut serliazer)
                    .expect("Error serializing value");

                let value_str =
                    String::from_utf8(buffer).expect("Error converting buffer to String");

                if i > 0 {
                    ordered_json_str.push_str(",\n");
                }
                ordered_json_str.push_str(&format!(
                    "   \"{}\': {}",
                    key,
                    value_str.replace("\n", "\n   ")
                ));
            }
        }

        ordered_json_str.push_str("\n}");
        return ordered_json_str;
    } else {
        String::from("Expected top-level JSON object")
    }
}

//Matches against the None and Some enums of the Option<T> type
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), //Executes code based on a particular enum match
    }
}
pub fn idk_anymore() {
    let five: Option<i32> = Some(5); //Binds a variable with data in an Option<i32> type
    let none: Option<i32> = None;
    plus_one(five);
    plus_one(Some(11));
    plus_one(none);
    plus_one(None);

    let answer: String = match plus_one(five) {
        Some(a) => format!("The answer is: {a}"),
        None => format!("Sorry, I didn't find anything"),
    };
    println!("{answer}");
}
