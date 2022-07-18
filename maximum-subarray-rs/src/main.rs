// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/maximum-subarray/

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix = nums;
        prefix.iter_mut().fold(0, |acc, n| {
            *n += acc;
            *n
        });

        prefix.iter().scan(0, |lowest, &n| {
            let old = *lowest;
            *lowest = std::cmp::min(*lowest, n);
            Some(n - old)
        }).max().unwrap_or(i32::MIN)
    }
}

fn main() {
    println!("{}", Solution::max_sub_array(vec![-2, -1, -3, 4, -1, 2, 1, -5, 4]));
}
