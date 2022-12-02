use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
        1 => problem_01(filename),
        _ => std::process::exit(1),
    }
}

fn problem_01<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut elves = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut sum: i32 = 0;
        let mut elf: i32 = 1;
        for line in lines {
            if let Ok(ip) = line {
                if ip.chars().count() == 0 {
                    elves.push((sum, elf));
                    sum = 0;
                    elf = elf + 1;
                } else {
                    sum = sum + ip.parse::<i32>().unwrap();
                }
            }
        }

        elves.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        println!("Elf: {}, Calories: {}", elves[0].0, elves[0].1);
        println!("Elf: {}, Calories: {}", elves[1].0, elves[1].1);
        println!("Elf: {}, Calories: {}", elves[2].0, elves[2].1);
        let total_cals = elves[0].0 + elves[1].0 + elves[2].0;
        println!("Total Calories: {}", total_cals)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
