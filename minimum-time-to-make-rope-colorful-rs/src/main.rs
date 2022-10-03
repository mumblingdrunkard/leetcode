// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/minimum-time-to-make-rope-colorful/

struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let o_sum = needed_time.iter().sum::<i32>();
        let mut groups = vec![];
        let mut prev = b' ';
        colors
            .bytes()
            .zip(needed_time.into_iter())
            .for_each(|(col, t)| {
                if col != prev {
                    prev = col;
                    groups.push((1, t));
                } else if let Some((cnt, gt)) = groups.last_mut() {
                    *cnt += 1;
                    *gt = std::cmp::max(*gt, t);
                }
            });

        o_sum - groups.into_iter().map(|(_, t)| t).sum::<i32>()
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5])
    );

    println!(
        "{}",
        Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1])
    );
}
