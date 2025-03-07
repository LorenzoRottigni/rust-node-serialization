use serde_json::Value;
use std::fs;

fn main() {
    // Read the JSON file
    let file_content =
        fs::read_to_string("./complexObject.json").expect("Failed to read JSON file");

    // Parse JSON into serde_json::Value
    let json: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // Start recursive traversal
    println!("JSON Structure:");
    traverse_json(&json, 0);
}

// Recursive function to traverse JSON and print key-value types
fn traverse_json(value: &Value, depth: usize) {
    let indent = " ".repeat(depth * 4); // Indentation for nested levels

    match value {
        Value::Object(map) => {
            for (key, val) in map {
                println!("{}Key: {}, Type: Object", indent, key);
                traverse_json(val, depth + 1);
            }
        }
        Value::Array(arr) => {
            println!("{}Type: Array [{} elements]", indent, arr.len());
            for (i, item) in arr.iter().enumerate() {
                println!("{}  Index {}:", indent, i);
                traverse_json(item, depth + 2);
            }
        }
        Value::String(_) => println!("{}Type: String", indent),
        Value::Number(_) => println!("{}Type: Number", indent),
        Value::Bool(_) => println!("{}Type: Boolean", indent),
        Value::Null => println!("{}Type: Null", indent),
    }
}
