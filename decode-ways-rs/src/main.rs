// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/decode-ways/

struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if let Some(b'0') = s.bytes().next() {
            return 0;
        }

        let (mut a, mut b, mut c) = (1, 1, 0);

        let s = s.as_bytes();

        for w in s.windows(2) {
            if let b'1'..=b'9' = w[1] {
                c += b;
            }

            if w >= &[b'1', b'0'] && w <= &[b'2', b'6'] {
                c += a;
            }

            (a, b, c) = (b, c, 0);
        }

        b
    }
}

fn main() {
    println!("{}", Solution::num_decodings("226".to_string()));
}
