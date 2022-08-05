// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/combination-sum-iv/

struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // ignore numbers larger than target and convert to usize
        let mut nums: Vec<_> = nums.into_iter().filter_map(|n| (n <= target).then(|| n as usize)).collect();
        nums.sort();

        let target = target as usize; // make target a usize to simplify usage later
        let mut combinations = vec![0; target + 1];

        // initial count of each number
        for &n in &nums {
            combinations[n] += 1;
        }

        // add combinations
        for i in 1..target + 1 {
            combinations[i] += nums
                .iter()
                // scan until n >= i, after which no combinations can be made
                // a number k >= n can never combine with another positive number to make n
                .scan(0, |_, &n| {
                    // a number k < n can combine with a number l = n - k to make n
                    (n < i).then(|| combinations[i - n])
                })
                .sum::<i32>();
        }

        combinations[target]
    }
}

fn main() {
    println!(
        "{}",
        Solution::combination_sum4(vec![3, 4, 5, 6, 7, 8, 9, 10], 10)
    );
}
