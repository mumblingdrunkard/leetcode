// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/utf-8-validation/

struct Solution {}

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut bytes = data.into_iter().map(|i| (i & 0xFF) as u8);
        while let Some(b) = bytes.next() {
            if b & 0b1000_0000 == 0 {
                // normal character, keep moving
                continue;
            }

            let n = b.leading_ones();

            if n > 4 || n == 1 {
                return false;
            }

            for _ in 0..n - 1 {
                match bytes.next() {
                    Some(b) => {
                        if b >> 6 != 0b10 {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}
