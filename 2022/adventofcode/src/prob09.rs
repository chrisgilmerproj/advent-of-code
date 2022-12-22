use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut bridge: Vec<Vec<u32>> = vec![vec![0; 6]; 5];
    let mut pos_h: Vec<i32> = vec![0, 0];
    let mut pos_t: Vec<i32> = vec![0, 0];
    if let Ok(lines) = read_lines(filename) {
        print_bridge(&bridge, &pos_h, &pos_t);
        for line in lines {
            if let Ok(ip) = line {
                let direction_steps: Vec<&str> = ip.split(" ").collect();
                let direction: &str = direction_steps[0];
                let steps: i32 = direction_steps[1].parse::<i32>().unwrap();
                println!("\n== {} {} ==", direction, steps);

                for i in 1..steps + 1 {
                    println!("====================");
                    println!("\ndirection {}, step {}", direction, i);
                    print_bridge(&bridge, &pos_h, &pos_t);
                    println!("");
                    println!("move H\n");
                    // Move H
                    match direction {
                        "U" => pos_h[0] += 1,
                        "D" => pos_h[0] -= 1,
                        "R" => pos_h[1] += 1,
                        "L" => pos_h[1] -= 1,
                        &_ => todo!(),
                    }
                    print_bridge(&bridge, &pos_h, &pos_t);
                    println!("");

                    // Move T
                    let diff_0 = pos_t[0] - pos_h[0];
                    let diff_1 = pos_t[1] - pos_h[1];
                    println!("\ndiffs: {}, {}", diff_0, diff_1);
                    let mut pos_delta_0 = 0;
                    let mut pos_delta_1 = 0;

                    match (diff_0, diff_1) {
                        (0, 0)
                        | (0, 1)
                        | (0, -1)
                        | (1, 0)
                        | (-1, 0)
                        | (-1, 1)
                        | (1, -1)
                        | (1, 1)
                        | (-1, -1) => {
                            pos_delta_0 = 0;
                            pos_delta_1 = 0
                        }
                        (0, 2) => {
                            pos_delta_0 = 0;
                            pos_delta_1 = -1;
                        }
                        (0, -2) => {
                            pos_delta_0 = 0;
                            pos_delta_1 = 1;
                        }
                        (2, 0) => {
                            pos_delta_0 = -1;
                            pos_delta_1 = 0;
                        }
                        (-2, 0) => {
                            pos_delta_0 = 1;
                            pos_delta_1 = 0;
                        }
                        (-2, -1) => {
                            pos_delta_0 = 1;
                            pos_delta_1 = 1;
                        }
                        (-1, 2) => {
                            pos_delta_0 = 1;
                            pos_delta_1 = -1;
                        }
                        (1, -2) => {
                            pos_delta_0 = -1;
                            pos_delta_1 = 1;
                        }
                        (1, 2) => {
                            pos_delta_0 = -1;
                            pos_delta_1 = -1;
                        }
                        // -----
                        (i32::MIN..=-3_i32, _) | (2_i32..=i32::MAX, _) => todo!(),
                        _ => todo!(),
                    }

                    println!("delta T: {}, {}", pos_delta_0, pos_delta_1);
                    println!("move T\n");
                    pos_t[0] += pos_delta_0;
                    pos_t[1] += pos_delta_1;

                    bridge[pos_t[0] as usize][pos_t[1] as usize] = 1;
                    print_bridge(&bridge, &pos_h, &pos_t);
                    println!("");
                }
            }
        }
    }
}

pub fn print_bridge(bridge: &Vec<Vec<u32>>, pos_h: &Vec<i32>, pos_t: &Vec<i32>) {
    println!("pos H: {:?}", pos_h);
    println!("pos T: {:?}", pos_t);
    let mut count = 0;
    for j in (0..5).rev() {
        println!("");
        for k in 0..6 {
            if bridge[j as usize][k as usize] == 1 {
                count += 1;
            }
            if j == pos_h[0] && k == pos_h[1] {
                print!("H");
            } else if j == pos_t[0] && k == pos_t[1] {
                print!("T");
            } else if j == 0 && k == 0 {
                print!("s");
            } else if bridge[j as usize][k as usize] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    println!("\n\nCount: {}\n", count);
}
