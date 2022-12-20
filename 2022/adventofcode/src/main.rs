use std::env;
use std::path::Path;

mod prob01;
mod prob02;
mod prob03;
mod prob04;
mod prob05;
mod prob06;
mod prob07;
mod prob08;
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
        1 => prob01::problem(filename),
        2 => prob02::problem(filename),
        3 => prob03::problem(filename),
        4 => prob04::problem(filename),
        5 => prob05::problem(filename),
        6 => prob06::problem(filename),
        7 => prob07::problem(filename),
        8 => prob08::problem(filename),
        _ => std::process::exit(1),
    }
}
