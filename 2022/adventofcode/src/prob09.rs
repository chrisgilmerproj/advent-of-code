use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut bridge: Vec<Vec<u32>> = vec![vec![0; 5]; 5];
    let mut pos_h: Vec<i32> = vec![0, 0];
    let mut pos_t: Vec<i32> = vec![0, 0];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let direction_steps: Vec<&str> = ip.split(" ").collect();
                let direction: &str = direction_steps[0];
                let steps: i32 = direction_steps[1].parse::<i32>().unwrap();
                println!("\n{} {}", direction, steps);

                for i in 1..steps + 1 {
                    println!("\nstep {}", i);
                    // Move H
                    match direction {
                        "U" => pos_h[0] += 1,
                        "D" => pos_h[0] -= 1,
                        "R" => pos_h[1] += 1,
                        "L" => pos_h[1] -= 1,
                        &_ => todo!(),
                    }
                    println!("pos H: {:?}", pos_h);
                    // Move T
                    println!(
                        "diffs {:?}: {} {}",
                        pos_t,
                        pos_t[0] - pos_h[0],
                        pos_t[1] - pos_h[1]
                    );
                    if pos_t[0] - pos_h[0] >= 2 {
                        pos_t[0] -= 1;
                    } else if pos_t[0] - pos_h[0] <= -2 {
                        pos_t[0] += 1;
                    } else if pos_t[1] - pos_h[1] >= 2 {
                        pos_t[1] -= 1;
                    } else if pos_t[1] - pos_h[1] <= -2 {
                        pos_t[1] += 1;
                    }
                    println!("pos T: {:?}", pos_t);
                    bridge[pos_t[0] as usize][pos_t[1] as usize] = 1;
                    for j in (0..5).rev() {
                        print!("\n");
                        for k in 0..5 {
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
                }
            }
        }
    }
    println!("{:?}", bridge)
}
