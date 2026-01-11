use std::{env, fs, process};

fn main() {
    let check_only = env::args().any(|a| a == "--check");

    let content = fs::read_to_string("swagger.v1.json")
        .expect("Failed to read swagger.v1.json");

    let value: serde_json::Value = serde_json::from_str(&content)
        .expect("Failed to parse JSON");

    let formatted = serde_json::to_string_pretty(&value)
        .expect("Failed to format JSON");

    if check_only {
        if content.trim() != formatted.trim() {
            eprintln!("swagger.v1.json is not formatted. Run: cargo run -p format-swagger");
            process::exit(1);
        }
        println!("swagger.v1.json formatting OK");
    } else {
        fs::write("swagger.v1.json", formatted + "\n")
            .expect("Failed to write swagger.v1.json");
        println!("Formatted swagger.v1.json");
    }
}
