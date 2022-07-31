// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/plus-one/

struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: Vec<_> = digits.into_iter().rev().collect();
        let mut rem = 1;

        for d in digits.iter_mut() {
            *d += rem;
            rem = *d / 10;
            *d %= 10;
        }

        if rem != 0 {
            digits.push(rem);
        }

        digits.into_iter().rev().collect()
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![9, 9, 9, 9]));
}
