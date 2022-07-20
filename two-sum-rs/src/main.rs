// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/two-sum/

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<_> = nums.into_iter().enumerate().collect();
        nums.sort_by(|(_, a), (_, b)| a.cmp(b));
        let (mut i, mut j) = (0, nums.len() - 1);
        while nums[i].1 + nums[j].1 != target {
            if nums[i].1 + nums[j].1 < target {
                i += 1;
            } else {
                j -= 1;
            }
        }

        vec![nums[i].0 as i32, nums[j].0 as i32]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![1, 2, 3, 4], 7));
}
