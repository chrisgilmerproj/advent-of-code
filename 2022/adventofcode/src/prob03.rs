use std::collections::HashSet;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut priority = 0;
    let mut badge_priority = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut count = 0;
        let mut badge_set: HashSet<char> = HashSet::new();
        for line in lines {
            if let Ok(ip) = line {
                let size = ip.chars().count();
                let setall: Vec<char> = ip.chars().collect();
                let sethash: HashSet<char> = HashSet::from_iter(setall);
                if count == 0 {
                    badge_set = sethash;
                } else {
                    let same: Vec<&char> = badge_set.intersection(&sethash).collect();
                    let mut newset: HashSet<char> = HashSet::new();
                    for s in same {
                        newset.insert(*s);
                    }
                    badge_set = newset;
                }

                let set01: Vec<char> = ip[..size / 2].chars().collect();
                let hash01: HashSet<char> = HashSet::from_iter(set01);
                let set02: Vec<char> = ip[size / 2..].chars().collect();
                let hash02: HashSet<char> = HashSet::from_iter(set02);
                let shared = hash01.intersection(&hash02);
                for item in shared {
                    priority += get_priority(*item);
                }
                count += 1;
                if count > 2 {
                    let group_badge = badge_set.clone().into_iter().collect::<Vec<char>>()[0];
                    badge_priority += get_priority(group_badge);
                    count = 0;
                }
            }
        }
    }
    println!("Value of priority is: {}", priority);
    println!("Badge priority is: {}", badge_priority);
}

fn get_priority(letter: char) -> usize {
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return alpha.chars().position(|c| c == letter).unwrap() + 1;
}
