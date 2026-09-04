use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &String = if args.len() > 1 {
        &args[1]
    } else {
        &String::from(".")
    };
}
