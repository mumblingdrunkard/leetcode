// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/

struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k == 0 {
            return 0;
        }

        let mut dp = vec![(i32::MAX, 0); k as usize + 1];

        for &p in &prices {
            for i in 1..dp.len() {
                dp[i].0 = std::cmp::min(dp[i].0, p - dp[i - 1].1);
                dp[i].1 = std::cmp::max(dp[i].1, p - dp[i].0);
            }
        }

        dp.last().unwrap().1
    }
}

fn main() {
    println!("{}", Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
}
