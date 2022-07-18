// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at
// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/

struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;

        // prefix is padded with 0s on the top and left side to simplify calculating the prefix
        let mut prefix = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        prefix
            .iter_mut()
            .skip(1)
            .zip(matrix.iter())
            .for_each(|(pr, mr)| {
                pr.iter_mut()
                    .skip(1)
                    .zip(mr.iter())
                    .for_each(|(p, &m)| *p = m)
            });

        // calculate the prefix matrix
        for r in 1..prefix.len() {
            for c in 1..prefix[r].len() {
                prefix[r][c] =
                    prefix[r - 1][c] + prefix[r][c - 1] + prefix[r][c] - prefix[r - 1][c - 1];
            }
        }

        // how many matrices have been found
        let mut found = 0;

        for upper in 0..prefix.len() - 1 {
            for lower in upper + 1..prefix.len() {
                let mut map = HashMap::new(); // map to track how many of each sum have been found
                map.insert(0, 1); // any sum - target = 0 is valid
                for i in 1..prefix[0].len() {
                    // calculates the sum of the box until this point
                    let row_sum = prefix[lower][i] - prefix[upper][i];
                    // the current sum can be combined with any other sum equal to sum - target
                    found += map.get(&(row_sum - target)).unwrap_or(&0);
                    // add one more of this sum to the map
                    map.insert(row_sum, map.get(&row_sum).unwrap_or(&0) + 1);
                }
            }
        }

        found
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0)
    );
}
