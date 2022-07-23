// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/integer-to-roman/

struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mappings = vec![
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ];

        { 0.. }
            .scan(num, |num, _| {
                (*num > 0).then(|| {
                    let i = mappings.partition_point(|x| x.0 <= *num) - 1;
                    *num -= mappings[i].0;
                    mappings[i].1
                })
            })
            .collect()
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(1994));
}
