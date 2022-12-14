use std::collections::HashSet;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    let search_size = 14;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                let mut buffer: Vec<char> = Vec::new();
                for (i, letter) in ip.chars().enumerate() {
                    if buffer.len() == search_size {
                        buffer.remove(0);
                    }
                    buffer.push(letter);
                    let signal: HashSet<char> = HashSet::from_iter(buffer.iter().cloned());
                    // println!("{} {:?} {:?}", letter, buffer, signal);
                    if signal.len() == search_size {
                        println!("Signal found starting at {}", i + 1);
                        break;
                    }
                }
            }
        }
    }
}
