use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No file provided as argument.");
        return;
    }
    let filename = &args[1];
    match fs::read_to_string(filename) {
        Ok(e) => match serde_yaml::from_str::<serde_yaml::Value>(&e) {
            Ok(_) => println!("The YAML file is valid !"),
            Err(e) => eprintln!("The YAML file is not valid : {}.", e),
        },
        Err(e) => eprintln!("Error reading file : {}.", e),
    }
}
