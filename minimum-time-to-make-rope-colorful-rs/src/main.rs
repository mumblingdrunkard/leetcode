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
        use std::cmp::Reverse;
        let mut v = colors
            .bytes()
            .zip(needed_time.into_iter())
            .scan((b' ', 0), |(prev_col, group), (b, t)| {
                if b != *prev_col {
                    *prev_col = b;
                    *group += 1;
                }
                Some((*group, Reverse(t)))
            })
            .collect::<Vec<_>>();

        let o_sum = v.iter().map(|(_, Reverse(t))| t).sum::<i32>();
        v.sort();
        v.dedup_by(|a, b| a.0 == b.0);
        let n_sum = v.iter().map(|(_, Reverse(t))| t).sum::<i32>();

        o_sum - n_sum
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
