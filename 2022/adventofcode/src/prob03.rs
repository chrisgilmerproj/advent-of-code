use std::collections::HashSet;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem_03<P>(filename: P)
where
    P: AsRef<Path>,
{
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let size = ip.chars().count();
                let set01: Vec<char> = ip[..size / 2].chars().collect();
                let hash01: HashSet<char> = HashSet::from_iter(set01);
                let set02: Vec<char> = ip[size / 2..].chars().collect();
                let hash02: HashSet<char> = HashSet::from_iter(set02);
                let shared = hash01.intersection(&hash02);
                for item in shared {
                    let value = alpha.chars().position(|c| c == *item).unwrap() + 1;
                    priority += value;
                    println!("{} {}", item, value);
                }
            }
        }
    }
    println!("Value of priority is: {}", priority);
}
