// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/zigzag-conversion/

struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut res = Vec::with_capacity(s.len());

        {
            let chars = s.as_bytes();
            let step = num_rows * 2 - 2;

            let mut left = 0;
            while left < chars.len() {
                res.push(chars[left]);
                left += step;
            }

            for r in 1..(num_rows - 1) {
                let mut left = r;
                let mut right = step - r;
                while left < chars.len() {
                    res.push(chars[left]);
                    if right < chars.len() {
                        res.push(chars[right]);
                    }
                    left += step;
                    right += step;
                }
            }

            left = num_rows - 1;
            while left < chars.len() {
                res.push(chars[left]);
                left += step;
            }
        }

        String::from_utf8(res).unwrap()
    }
}

fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
}
