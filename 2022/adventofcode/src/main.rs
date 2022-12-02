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
        2 => problem_02(filename),
        _ => std::process::exit(1),
    }
}

fn problem_01<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut elves = Vec::new();

    if let Ok(lines) = read_lines(filename) {
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

// Problem Comments
// A,X - Rock (1)
// B,Y - Paper (2)
// C,Z - Scissors (3)
// Lose (0) - X
// Draw (3) - Y
// Win (6) - Z
fn problem_02<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut points_l: i32 = 0;
    let mut points_r: i32 = 0;
    let mut points_strategy: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "A X" => {
                        // Lose to Rock with Scissors
                        points_l = points_l + 1 + 3;
                        points_r = points_r + 1 + 3;
                        points_strategy = points_strategy + 3 + 0;
                    }
                    "A Y" => {
                        // Draw to Rock with Rock
                        points_l = points_l + 1 + 0;
                        points_r = points_r + 2 + 6;
                        points_strategy = points_strategy + 1 + 3;
                    }
                    "A Z" => {
                        // Win to Rock with Paper
                        points_l = points_l + 1 + 6;
                        points_r = points_r + 3 + 0;
                        points_strategy = points_strategy + 2 + 6;
                    }
                    "B X" => {
                        // Lose to Paper with Rock
                        points_l = points_l + 2 + 6;
                        points_r = points_r + 1 + 0;
                        points_strategy = points_strategy + 1 + 0;
                    }
                    "B Y" => {
                        // Draw to Paper with Paper
                        points_l = points_l + 2 + 3;
                        points_r = points_r + 2 + 3;
                        points_strategy = points_strategy + 2 + 3;
                    }
                    "B Z" => {
                        // Win to Paper with Scissors
                        points_l = points_l + 2 + 0;
                        points_r = points_r + 3 + 6;
                        points_strategy = points_strategy + 3 + 6;
                    }
                    "C X" => {
                        // Lose to Scissors with Paper
                        points_l = points_l + 3 + 0;
                        points_r = points_r + 1 + 6;
                        points_strategy = points_strategy + 2 + 0;
                    }
                    "C Y" => {
                        // Draw to Scissors with Scissors
                        points_l = points_l + 3 + 6;
                        points_r = points_r + 2 + 0;
                        points_strategy = points_strategy + 3 + 3;
                    }
                    "C Z" => {
                        // Win to Scissors with Rock
                        points_l = points_l + 3 + 3;
                        points_r = points_r + 3 + 3;
                        points_strategy = points_strategy + 1 + 6;
                    }
                    _ => continue,
                }
            }
        }
    }
    println!("Points for opponent: {}", points_l);
    println!("Points for you: {}", points_r);
    println!("Points for correct strategy: {}", points_strategy);
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
