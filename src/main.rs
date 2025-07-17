use std::{env, fs, process};

use rijson::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: must provide a json file");
        eprintln!("Use: {} <archivo.json>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let contents = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error leyendo el archivo '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    let p = Parser::new(contents.chars().collect());
    for result in p {
        match result {
            Ok(json_value) => println!("{json_value}"),
            Err(parse_error) => eprintln!("Error parsing JSON: {}", parse_error),
        }
    }
}
