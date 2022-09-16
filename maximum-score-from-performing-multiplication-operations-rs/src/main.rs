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
        use std::cmp;
        let (n, m) = (nums, multipliers);

        let (mut buf1, mut buf2) = (vec![i32::MIN; m.len() + 1], vec![i32::MIN; m.len() + 1]);
        let (mut curr, mut next) = (&mut buf1, &mut buf2);

        curr[0] = 0;
        for t in 0..m.len() {
            for l in 0..t + 1 {
                // from current position, check if either of the next positions are greater than
                // their already calculated maxima

                // next position with same l (takes from right)
                next[l] = cmp::max(
                    next[l],                                   // current max for next position
                    curr[l] + n[n.len() - 1 - (t - l)] * m[t], // position + multiplier * right number
                );

                // next position with l + 1 (takes from left)
                next[l + 1] = cmp::max(
                    next[l + 1],           // current max for next position
                    curr[l] + n[l] * m[t], // position + multiplier * left number
                );
            }

            curr[..t + 2].fill(i32::MIN); // reset curr
            (curr, next) = (next, curr); // swap the buffers
        }

        // result is max of last turn
        *curr.iter().max().unwrap()
    }
}

fn main() {
    println!("{}", Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]));

    println!(
        "{}",
        Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6])
    );
}
