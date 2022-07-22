// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/first-missing-positive/

struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut i = 0;
        while i < nums.len() {
            // if this slot is 0
            if nums[i] == 0 {
                i += 1;
                continue;
            }

            let n = nums[i];
            if n > 0 && n as usize <= nums.len() {
                // if n - 1 lies within the array

                if n as usize - 1 == i {
                    // if this is already the slot for n
                    i += 1;
                    continue;
                }

                // swap the numbers so that n goes to its spot
                let mut tmp = nums[n as usize - 1];
                if tmp == n {
                    // if the number at the position is already n
                    tmp = 0;
                }
                nums[n as usize - 1] = n;
                nums[i] = tmp;
            } else {
                // number is outside range and we can ignore it
                nums[i] = 0;
            }
        }

        // scan through values of n to find the first missing n
        for n in 1..nums.len() + 1 {
            if nums[n - 1] != n as i32 {
                return n as i32;
            }
        }

        // all values inside the array were found
        return nums.len() as i32 + 1;
    }
}

fn main() {
    println!("{}", Solution::first_missing_positive(vec![1, 2, 0]));
}
