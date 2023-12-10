use json::{object, JsonValue};

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
