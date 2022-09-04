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
// https://leetcode.com/problems/numbers-with-same-consecutive-differences/

struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];

        for i in 1..=9 {
            res.append(&mut Self::nums_same_consec_diff_recursive(n - 1, k, i));
        }

        res
    }

    fn nums_same_consec_diff_recursive(n: i32, k: i32, lead: i32) -> Vec<i32> {
        if n == 0 {
            return vec![lead];
        }

        let mut res = vec![];
        let last_digit = lead % 10;

        if last_digit >= k {
            res.append(&mut Self::nums_same_consec_diff_recursive(
                n - 1,
                k,
                lead * 10 + last_digit - k,
            ));
        }

        if last_digit + k < 10 && k != 0 {
            res.append(&mut Self::nums_same_consec_diff_recursive(
                n - 1,
                k,
                lead * 10 + last_digit + k,
            ));
        }

        res
    }
}

fn main() {
    println!("{:?}", Solution::nums_same_consec_diff(2, 1));
}
