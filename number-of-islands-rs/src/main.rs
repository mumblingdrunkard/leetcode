// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/number-of-islands/

struct Solution {}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        use std::collections::VecDeque;
        let (m, n) = (grid.len(), grid[0].len());

        // surround grid with water to simplify calculation
        grid.insert(0, vec!['0'; n]);
        grid.push(vec!['0'; n]);
        for r in &mut grid {
            r.insert(0, '0');
            r.push('0');
        }

        let mut filled = vec![vec![false; n + 2]; m + 2];

        let mut islands = 0;

        for r in 1..=m {
            for c in 1..=n {
                if grid[r][c] == '0' || filled[r][c] {
                    continue;
                }

                islands += 1;

                let mut q = VecDeque::new();
                q.push_back((r, c));
                while let Some((r, c)) = q.pop_front() {
                    if grid[r][c] == '0' || filled[r][c] {
                        continue;
                    }

                    filled[r][c] = true;

                    q.push_back((r, c - 1));
                    q.push_back((r, c + 1));
                    q.push_back((r - 1, c));
                    q.push_back((r + 1, c));
                }
            }
        }

        islands
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ])
    );
}
