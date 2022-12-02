use std::path::Path;

use crate::utils::read_lines;

pub fn problem_01<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut elves = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let mut sum: i32 = 0;
        let mut elf: i32 = 1;
        for line in lines {
            if let Ok(ip) = line {
                if ip.chars().count() == 0 {
                    elves.push((sum, elf));
                    sum = 0;
                    elf = elf + 1;
                } else {
                    sum = sum + ip.parse::<i32>().unwrap();
                }
            }
        }

        elves.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        println!("Elf: {}, Calories: {}", elves[0].0, elves[0].1);
        println!("Elf: {}, Calories: {}", elves[1].0, elves[1].1);
        println!("Elf: {}, Calories: {}", elves[2].0, elves[2].1);
        let total_cals = elves[0].0 + elves[1].0 + elves[2].0;
        println!("Total Calories: {}", total_cals)
    }
}
