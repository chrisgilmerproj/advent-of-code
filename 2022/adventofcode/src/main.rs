use std::env;
use std::path::Path;

mod prob01;
mod prob02;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Not enough arguments passed, please include problem number and filename!");
        std::process::exit(1);
    }
    let problem_num = &args[1].parse::<i32>().unwrap();
    let filename = &args[2];
    if !Path::new(filename).exists() {
        println!("File does not exist at path {}", filename);
        std::process::exit(1);
    }
    println!("File: {}", filename);

    match problem_num {
        1 => prob01::problem_01(filename),
        2 => prob02::problem_02(filename),
        _ => std::process::exit(1),
    }
}
