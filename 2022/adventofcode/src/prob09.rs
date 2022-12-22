use std::collections::HashSet;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    //let num_knots = 2;
    let num_knots = 10;
    let mut knots: Vec<Vec<i32>> = Vec::new();
    for _i in 0..num_knots {
        knots.push(vec![0, 0]);
    }
    let mut visited: HashSet<Vec<i32>> = HashSet::new();
    visited.insert(knots.last().expect("Missing last value").to_vec());

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let direction_steps: Vec<&str> = ip.split(" ").collect();
                let direction: &str = direction_steps[0];
                let steps: i32 = direction_steps[1].parse::<i32>().unwrap();
                println!("\n== {} {} ==", direction, steps);

                for i in 1..steps + 1 {
                    println!("\ndirection {}, step {}", direction, i);
                    // Move Head Knot
                    match direction {
                        "U" => knots[0][0] += 1,
                        "D" => knots[0][0] -= 1,
                        "R" => knots[0][1] += 1,
                        "L" => knots[0][1] -= 1,
                        &_ => todo!(),
                    }

                    for k in 0..num_knots - 1 {
                        // Move Knots
                        let diff_0 = knots[k][0] - knots[k + 1][0];
                        let diff_1 = knots[k][1] - knots[k + 1][1];
                        println!("diffs: {}, {}", diff_0, diff_1);
                        if diff_0.abs() == 2 && diff_1 == 0 {
                            if diff_0 > 0 {
                                knots[k + 1][0] += 1;
                            } else if diff_0 < 0 {
                                knots[k + 1][0] -= 1;
                            }
                        } else if diff_0 == 0 && diff_1.abs() == 2 {
                            if diff_1 > 0 {
                                knots[k + 1][1] += 1;
                            } else if diff_1 < 0 {
                                knots[k + 1][1] -= 1;
                            }
                        } else if ((i32::pow(diff_0, 2) + i32::pow(diff_1, 2)) as f64).sqrt() > 2.0
                        {
                            if diff_0 > 0 {
                                knots[k + 1][0] += 1;
                            } else if diff_0 < 0 {
                                knots[k + 1][0] -= 1;
                            }
                            if diff_1 > 0 {
                                knots[k + 1][1] += 1;
                            } else if diff_1 < 0 {
                                knots[k + 1][1] -= 1;
                            }
                        }
                    }
                    visited.insert(knots.last().expect("Missing last value").to_vec());
                }
            }
        }
    }
    println!("\n\nCount: {}\n", visited.len());
}
