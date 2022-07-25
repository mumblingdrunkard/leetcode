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
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let first = nums.partition_point(|&x| x < target);
        let last = nums.partition_point(|&x| x < target + 1) - 1;
        println!("{}, {}", first, last);

        let first = match first < nums.len() && nums[first] == target {
            true => first as i32,
            false => -1,
        };

        let last = match last < nums.len() && nums[last] == target {
            true => last as i32,
            false => -1,
        };

        match (first, last) {
            (-1, _) | (_, -1) => vec![-1, -1],
            (f, l) => vec![f, l],
        }
    }
}

fn main() {
    println!("{:?}", Solution::search_range(vec![], 8));
}
