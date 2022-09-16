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
// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/

struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        use std::cmp::max;
        let (n, m) = (nums, multipliers);
        let mut s = vec![vec![i32::MIN; m.len() + 1]; m.len() + 1];

        s[0][0] = 0; // s[t][l] is the max result at turn t with l elements from the left and t-l
                     // elements from the right

        for t in 0..m.len() {
            for l in 0..t + 1 {
                // from current position, check if either of the next positions are greater than
                // their already calculated maxima

                // next position with same l (takes from right)
                s[t + 1][l] = max(
                    // current max for next position
                    s[t + 1][l],
                    // current position + current multiplier * number from right
                    s[t][l] + n[n.len() - 1 - (t - l)] * m[t],
                );

                // next position with l + 1 (takes from left)
                s[t + 1][l + 1] = max(
                    s[t + 1][l + 1], // current max for next position
                    // current position + current multiplier * number from left
                    s[t][l] + n[l] * m[t],
                );
            }
        }

        // result is max of last turn
        *s.last().unwrap().iter().max().unwrap()
    }
}

fn main() {
    println!("{}", Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]));

    println!(
        "{}",
        Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6])
    );
}
