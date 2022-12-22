use std::collections::HashSet;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut pos_h: Vec<i32> = vec![0, 0];
    let mut pos_t: Vec<i32> = vec![0, 0];
    let mut visited: HashSet<Vec<i32>> = HashSet::new();
    visited.insert(pos_t.clone());

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let direction_steps: Vec<&str> = ip.split(" ").collect();
                let direction: &str = direction_steps[0];
                let steps: i32 = direction_steps[1].parse::<i32>().unwrap();
                println!("\n== {} {} ==", direction, steps);

                for i in 1..steps + 1 {
                    println!("\ndirection {}, step {}", direction, i);
                    // Move H
                    match direction {
                        "U" => pos_h[0] += 1,
                        "D" => pos_h[0] -= 1,
                        "R" => pos_h[1] += 1,
                        "L" => pos_h[1] -= 1,
                        &_ => todo!(),
                    }

                    // Move T
                    let diff_0 = pos_h[0] - pos_t[0];
                    let diff_1 = pos_h[1] - pos_t[1];
                    println!("diffs: {}, {}", diff_0, diff_1);
                    if diff_0.abs() > 1 || diff_1.abs() > 1 {
                        if diff_0 < 1 {
                            pos_t[0] -= 1;
                        } else if diff_0 > 1 {
                            pos_t[0] += 1;
                        }
                        if diff_1 < 1 {
                            pos_t[1] -= 1;
                        } else if diff_1 > 1 {
                            pos_t[1] += 1;
                        }
                        visited.insert(pos_t.clone());
                    }
                }
            }
        }
    }
    println!("\n\nCount: {}\n", visited.len());
}
