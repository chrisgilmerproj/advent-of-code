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
                for c in ip.chars() {
                    let i = c.to_digit(10).unwrap();
                    treeline.push(i);
                }
            }
            tree.push(treeline);
        }
        // Iterate through inside trees
        let mut max_viz: i32 = 0;
        for r in 0..rows {
            for c in 0..cols {
                let max = tree[r][c];
                let mut viz: i32 = 1;
                // Search the tree and break
                // println!("\nbegin: {}, {}, {}", r, c, tree[r][c]);
                // println!("visible: {}", count_visible);
                // up
                let mut found_block: i32 = 0;
                for (num, i) in (0..r).rev().enumerate() {
                    // println!("up   : {}, {}, {}, {}", i, c, tree[i][c], tree[i][c] >= max);
                    if i == 0 || tree[i][c] >= max {
                        // println!("Viz up    {} {}", viz, num as i32 + 1);
                        viz *= num as i32 + 1;
                    }
                    if tree[i][c] >= max {
                        found_block += 1;
                        break;
                    }
                }
                // down
                for (num, i) in (r + 1..rows).enumerate() {
                    // println!("down : {}, {}, {}, {}", i, c, tree[i][c], tree[i][c] >= max);
                    if i == rows - 1 || tree[i][c] >= max {
                        // println!("Viz down  {} {}", viz, num as i32 + 1);
                        viz *= num as i32 + 1;
                    }
                    if tree[i][c] >= max {
                        found_block += 1;
                        break;
                    }
                }
                // left
                for (num, i) in (0..c).rev().enumerate() {
                    // println!("left : {}, {}, {}, {}", r, i, tree[r][i], tree[r][i] >= max);
                    if i == 0 || tree[r][i] >= max {
                        // println!("Viz left  {} {}", viz, num as i32 + 1);
                        viz *= num as i32 + 1;
                    }
                    if tree[r][i] >= max {
                        found_block += 1;
                        break;
                    }
                }
                // right
                for (num, i) in (c + 1..cols).enumerate() {
                    // println!("right: {}, {}, {}, {}", r, i, tree[r][i], tree[r][i] >= max);
                    if i == cols - 1 || tree[r][i] >= max {
                        // println!("Viz right {} {}", viz, num as i32 + 1);
                        viz *= num as i32 + 1;
                    }
                    if tree[r][i] >= max {
                        found_block += 1;
                        break;
                    }
                }
                if found_block < 4 {
                    count_visible += 1;
                }
                if viz > max_viz {
                    max_viz = viz;
                }
            }
        }
        // println!("{:?}", tree);
        // println!("size: {} rows, {} cols", rows, cols);
        println!("visible: {} trees", count_visible);
        println!("max viz: {}", max_viz);
    }
}
