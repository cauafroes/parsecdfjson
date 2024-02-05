use std::{env, fs};
use serde_json::{Result, Value};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    // println!("Path: {}", path);

    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    let v: Value = serde_json::from_str(&contents)?;

    println!("a: {}", v["name"]);

    Ok(())
}
