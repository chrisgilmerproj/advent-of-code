use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let mut register: i32 = 1;
        let mut cycle_counter: i32 = 0;
        let important_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
        let mut register_sum: i32 = 0;
        let mut crt: Vec<String> = Vec::new();
        let screen_size: usize = 40;
        for line in lines {
            if let Ok(ip) = line {
                let instructions: Vec<&str> = ip.split(" ").collect();
                let mut cycles: i32 = 0;
                let mut val: i32 = 0;
                match instructions[0] {
                    "noop" => {
                        cycles += 1;
                    }
                    "addx" => {
                        cycles += 2;
                        val = instructions[1].parse::<i32>().unwrap();
                    }
                    &_ => todo!(),
                }
                for i in 0..cycles {
                    cycle_counter += 1;
                    if (register - (crt.len() as i32 % screen_size as i32)).abs() <= 1 {
                        crt.push("#".to_string());
                    } else {
                        crt.push(".".to_string());
                    }
                    if i > 0 {
                        register += val;
                    }
                    if important_cycles.contains(&cycle_counter) {
                        println!(
                            "{} {} {}",
                            cycle_counter,
                            register,
                            cycle_counter * register
                        );
                        register_sum += cycle_counter * register;
                    }
                }
            }
        }
        // Print the screen
        let mut screen_marker: usize = 0;
        println!("\nScreen: ");
        while screen_marker < crt.len() {
            if crt.len() - screen_marker <= screen_size {
                println!("b {}", crt[screen_marker..].join(""));
            } else {
                println!(
                    "a {}",
                    crt[screen_marker..screen_marker + screen_size - 1].join("")
                );
            }
            screen_marker += screen_size;
        }
        println!("Register Sum: {}", register_sum);
    }
}
