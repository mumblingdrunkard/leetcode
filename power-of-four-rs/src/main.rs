// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/power-of-four/

struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
    }
}

fn main() {
    println!("{}", Solution::is_power_of_four(1));
    println!("{}", Solution::is_power_of_four(4));
    println!("{}", Solution::is_power_of_four(16));
    println!("{}", Solution::is_power_of_four(15));
    println!("{}", Solution::is_power_of_four(8));
}
