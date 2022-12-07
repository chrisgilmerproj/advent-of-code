use regex::Regex;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut found_commands: bool = false;
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    found_commands = true;
                    continue;
                }
                if !found_commands {
                    for (i, c) in ip.chars().enumerate() {
                        if c.is_alphabetic() {
                            while grid.len() < i / 4 + 1 {
                                let row: Vec<char> = Vec::new();
                                grid.push(row);
                            }
                            grid[i / 4].insert(0, c);
                        }
                    }
                } else {
                    let caps = re.captures(&ip).unwrap();
                    let mut num: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let col1: i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap() - 1;
                    let col2: i32 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap() - 1;

                    let mut crane: Vec<char> = Vec::new();
                    while num > 0 {
                        let thing = grid[col1 as usize].pop().unwrap();
                        crane.insert(0, thing);
                        // grid[col2 as usize].push(thing);
                        num -= 1;
                    }
                    for thing in crane {
                        grid[col2 as usize].push(thing);
                    }
                }
            }
        }
        println!("grid: {:?}", grid);
        let mut final_string = String::from("");
        for row in grid {
            final_string.push(*row.last().unwrap());
        }
        println!("Final Crates On Top: {}", final_string);
    }
}
