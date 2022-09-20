// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/maximum-length-of-repeated-subarray/

struct Solution {}

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut longest = 0;
        for i in 0..nums2.len() {
            let zipped = nums1.iter().zip(nums2[i..].iter());
            let mut len = 0;
            for (a, b) in zipped {
                if a == b {
                    len += 1;
                } else {
                    len = 0;
                }

                longest = std::cmp::max(longest, len);
            }
        }

        for i in 0..nums1.len() {
            let zipped = nums1[i..].iter().zip(nums2.iter());
            let mut len = 0;
            for (a, b) in zipped {
                if a == b {
                    len += 1;
                } else {
                    len = 0;
                }

                longest = std::cmp::max(longest, len);
            }
        }

        longest
    }
}
fn main() {
    println!(
        "{}",
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
    );

    println!(
        "{}",
        Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0])
    );
}
