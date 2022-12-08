use std::collections::HashMap;
use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let mut tree: Vec<String> = Vec::new();
        let mut dir_size: HashMap<String, i32> = HashMap::new();
        for line in lines {
            // println!("{:?}", tree);
            if let Ok(ip) = line {
                if ip.starts_with("$ ") {
                    let command_line = ip.split(" ").collect::<Vec<&str>>();
                    match command_line[1] {
                        "cd" => {
                            if command_line[2] != ".." {
                                let mut previous: String = String::from("");
                                if tree.len() > 0 {
                                    previous = tree.last().unwrap().to_string();
                                }
                                if command_line[2].to_string() == "/" {
                                    tree.push(command_line[2].to_string());
                                } else {
                                    if previous == "/" {
                                        tree.push(previous + &command_line[2].to_string());
                                    } else {
                                        tree.push(previous + "/" + &command_line[2].to_string());
                                    }
                                }
                            } else {
                                tree.pop();
                            }
                        }
                        "ls" => {}
                        _ => std::process::exit(1),
                    }
                } else {
                    if !ip.starts_with("dir") {
                        let filesize = ip.split(" ").collect::<Vec<&str>>();
                        let size = filesize[0].parse::<i32>().unwrap();
                        for dir in tree.clone() {
                            if dir_size.contains_key(dir.as_str()) {
                                let cur_size = dir_size.get(dir.as_str()).unwrap();
                                dir_size.insert(dir, cur_size + size);
                            } else {
                                dir_size.insert(dir, size);
                            }
                        }
                    }
                }
            }
        }
        // println!("{:?} {:?}", tree, dir_size);

        println!("\nMatching Directories:");
        let mut sum = 0;
        for (key, value) in dir_size.iter() {
            if value <= &100000 {
                println!("{} {}", key, value);
                sum += value;
            }
        }
        println!("\nTotal: {}", sum);
        println!(
            "Total of all files: {:?}",
            dir_size.get(&String::from("/")).unwrap()
        );
    }
}
