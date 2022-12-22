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
        for line in lines {
            if let Ok(ip) = line {
                let instructions: Vec<&str> = ip.split(" ").collect();
                // println!("{:?}", instructions);
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
                    if i > 0 {
                        register += val;
                    }
                    cycle_counter += 1;
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
        println!("Register Sum: {}", register_sum);
    }
}
