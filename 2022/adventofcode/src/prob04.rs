use std::path::Path;

use crate::utils::read_lines;

pub fn problem_04<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut counter = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(",").collect::<Vec<&str>>();
                let group_a = split_range(split[0]);
                let group_b = split_range(split[1]);
                if contains(group_a.clone(), group_b.clone())
                    || contains(group_b.clone(), group_a.clone())
                {
                    counter += 1;
                }
            }
        }
    }
    println!("Overlaps: {}", counter);
}

fn split_range(range: &str) -> Vec<i32> {
    let s = range.split("-").collect::<Vec<&str>>();
    let a: i32 = s[0].to_string().parse::<i32>().unwrap();
    let b: i32 = s[1].to_string().parse::<i32>().unwrap();
    let s_range = [a, b].to_vec();
    return s_range;
}

fn contains(group_a: Vec<i32>, group_b: Vec<i32>) -> bool {
    return group_a[0] >= group_b[0] && group_a[1] <= group_b[1];
}
