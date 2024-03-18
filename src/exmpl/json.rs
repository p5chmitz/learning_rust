#![allow(dead_code)]
#![allow(unused_variables)]

use json::{object, JsonValue};
use serde_json::{json, Value};

//JSON parsing experiment
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

fn enrollmentREquestBuilder(crdNumber: &str) -> Vec<u8> {
    let enrollment = json!({
        "type": "REGISTRATION_INDIVIDUAL",
        "principalApprover": {
            "crdNumber": crdNumber,
        }
    });
    let json_string = serde_json::to_string(&enrollment).unwrap();
    json_string.into_bytes()
}
