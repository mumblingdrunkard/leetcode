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
// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/

struct Solution {}

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const M: i64 = 1000000007;
        (1..=n as i64).fold(0, |res, n| {
            let len = 64 - n.leading_zeros();
            ((res << len) | n) % M
        }) as i32
    }
}

fn main() {
    println!("{}", Solution::concatenated_binary(12));
}
