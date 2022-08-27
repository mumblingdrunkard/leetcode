// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/max-consecutive-ones-ii/

struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = 0;
        let mut i = 0;
        let mut j = 0;
        let mut zeros = 0;
        while i < nums.len() && j < nums.len() {
            // collect until nums[j..i] contains k zeros
            while zeros < k && i < nums.len() {
                if nums[i] == 0 {
                    zeros += 1;
                }
                i += 1;
            }

            // scan until next zero
            while i < nums.len() && nums[i] != 0 {
                i += 1;
            }

            // nums[j..i] now contains k zeros and is the longest possible for j
            max = std::cmp::max(i - j, max);

            // move j until we have removed one zero
            while j < nums.len() && nums[j] != 0 {
                j += 1;
            }

            j += 1;
            zeros -= 1;
        }

        max as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        )
    );
}
