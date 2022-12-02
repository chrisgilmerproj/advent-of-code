use std::path::Path;

use crate::utils::read_lines;

// Problem Comments
// A,X - Rock (1)
// B,Y - Paper (2)
// C,Z - Scissors (3)
// Lose (0) - X
// Draw (3) - Y
// Win (6) - Z
pub fn problem_02<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut points_l: i32 = 0;
    let mut points_r: i32 = 0;
    let mut points_strategy: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "A X" => {
                        // Lose to Rock with Scissors
                        points_l = points_l + 1 + 3;
                        points_r = points_r + 1 + 3;
                        points_strategy = points_strategy + 3 + 0;
                    }
                    "A Y" => {
                        // Draw to Rock with Rock
                        points_l = points_l + 1 + 0;
                        points_r = points_r + 2 + 6;
                        points_strategy = points_strategy + 1 + 3;
                    }
                    "A Z" => {
                        // Win to Rock with Paper
                        points_l = points_l + 1 + 6;
                        points_r = points_r + 3 + 0;
                        points_strategy = points_strategy + 2 + 6;
                    }
                    "B X" => {
                        // Lose to Paper with Rock
                        points_l = points_l + 2 + 6;
                        points_r = points_r + 1 + 0;
                        points_strategy = points_strategy + 1 + 0;
                    }
                    "B Y" => {
                        // Draw to Paper with Paper
                        points_l = points_l + 2 + 3;
                        points_r = points_r + 2 + 3;
                        points_strategy = points_strategy + 2 + 3;
                    }
                    "B Z" => {
                        // Win to Paper with Scissors
                        points_l = points_l + 2 + 0;
                        points_r = points_r + 3 + 6;
                        points_strategy = points_strategy + 3 + 6;
                    }
                    "C X" => {
                        // Lose to Scissors with Paper
                        points_l = points_l + 3 + 0;
                        points_r = points_r + 1 + 6;
                        points_strategy = points_strategy + 2 + 0;
                    }
                    "C Y" => {
                        // Draw to Scissors with Scissors
                        points_l = points_l + 3 + 6;
                        points_r = points_r + 2 + 0;
                        points_strategy = points_strategy + 3 + 3;
                    }
                    "C Z" => {
                        // Win to Scissors with Rock
                        points_l = points_l + 3 + 3;
                        points_r = points_r + 3 + 3;
                        points_strategy = points_strategy + 1 + 6;
                    }
                    _ => continue,
                }
            }
        }
    }
    println!("Points for opponent: {}", points_l);
    println!("Points for you: {}", points_r);
    println!("Points for correct strategy: {}", points_strategy);
}
