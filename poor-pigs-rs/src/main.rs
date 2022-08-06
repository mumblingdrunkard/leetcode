// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/poor-pigs/

struct Solution {}

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let tests = minutes_to_test / minutes_to_die;
        let mut pigs = 0;
        let mut tested = 1;
        while tested < buckets {
            tested *= tests+1;
            pigs += 1;
        }
        pigs
    }
}

fn main() {
    println!("{}", Solution::poor_pigs(4, 15, 15));
    println!("{}", Solution::poor_pigs(1000, 15, 60));
}
