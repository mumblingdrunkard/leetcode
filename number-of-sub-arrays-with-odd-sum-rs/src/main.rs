// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/

struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        arr.iter()
            .scan((0, 0, 0), |(cums, evens, odds), n| {
                *cums += n;

                match *cums % 2 {
                    0 => {
                        *evens += 1;
                        Some(*odds)
                    }
                    1 => {
                        *odds += 1;
                        Some(*evens + 1)
                    }
                    _ => None,
                }
            })
            .fold(0, |sum, n| (sum + n) % 1000000007)
    }
}

fn main() {
    println!("{}", Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]));
}
