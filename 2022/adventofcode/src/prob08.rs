use std::path::Path;

use crate::utils::read_lines;

pub fn problem<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let mut count_visible: i32 = 0;
        let mut tree: Vec<Vec<u32>> = Vec::new();
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        for line in lines {
            let mut treeline: Vec<u32> = Vec::new();
            if let Ok(ip) = line {
                if rows == 0 {
                    cols = ip.chars().count();
                }
                rows += 1;
                for (num, c) in ip.chars().enumerate() {
                    let i = c.to_digit(10).unwrap();
                    treeline.push(i);
                    // Pre-count the visible trees on the edge
                    if num == 0 || num == cols - 1 {
                        count_visible += 1;
                    } else if rows == 1 || rows == cols {
                        count_visible += 1;
                    }
                }
            }
            tree.push(treeline);
        }
        // Iterate through inside trees
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                let max = tree[r][c];
                // Search the tree and break
                // println!("\nbegin: {}, {}, {}", r, c, tree[r][c]);
                // println!("visible: {}", count_visible);
                // up
                let mut found_up: bool = false;
                for i in (0..r).rev() {
                    // println!("up   : {}, {}, {}, {}", i, c, tree[i][c], tree[i][c] >= max);
                    if tree[i][c] >= max {
                        found_up = true;
                        break;
                    }
                }
                if !found_up {
                    count_visible += 1;
                    continue;
                }
                // down
                let mut found_down: bool = false;
                for i in r + 1..rows {
                    // println!("down : {}, {}, {}, {}", i, c, tree[i][c], tree[i][c] >= max);
                    if tree[i][c] >= max {
                        found_down = true;
                        break;
                    }
                }
                if !found_down {
                    count_visible += 1;
                    continue;
                }
                // left
                let mut found_left: bool = false;
                for i in (0..c).rev() {
                    // println!("left : {}, {}, {}, {}", r, i, tree[r][i], tree[r][i] >= max);
                    if tree[r][i] >= max {
                        found_left = true;
                        break;
                    }
                }
                if !found_left {
                    count_visible += 1;
                    continue;
                }
                // right
                let mut found_right: bool = false;
                for i in c + 1..cols {
                    // println!("right: {}, {}, {}, {}", r, i, tree[r][i], tree[r][i] >= max);
                    if tree[r][i] >= max {
                        found_right = true;
                        break;
                    }
                }
                if !found_right {
                    count_visible += 1;
                    continue;
                }
            }
        }
        // println!("{:?}", tree);
        // println!("size: {} rows, {} cols", rows, cols);
        println!("visible: {} trees", count_visible);
    }
}
