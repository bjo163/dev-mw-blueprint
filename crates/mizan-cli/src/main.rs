use mizan_engine::evaluate;
use mizan_model::MizanEvent;
use std::{env, fs, io::{self, Read}, process};

fn main() {
    let input = match env::args().nth(1) {
        Some(path) => fs::read_to_string(&path).unwrap_or_else(|err| {
            eprintln!("failed to read {path}: {err}");
            process::exit(2);
        }),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap_or_else(|err| {
                eprintln!("failed to read stdin: {err}");
                process::exit(2);
            });
            buf
        }
    };

    let event: MizanEvent = serde_json::from_str(&input).unwrap_or_else(|err| {
        eprintln!("invalid MizanEvent JSON: {err}");
        process::exit(2);
    });

    let result = evaluate(&event);
    let output = serde_json::to_string_pretty(&result).unwrap_or_else(|err| {
        eprintln!("failed to serialize result: {err}");
        process::exit(2);
    });

    println!("{output}");
}
