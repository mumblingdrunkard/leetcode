// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/palindrome-number/

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let digits: Vec<_> = (0..)
            .scan(x, |rest, _| {
                let (rem, old) = (*rest % 10, *rest);
                *rest /= 10;

                (old != 0).then(|| rem)
            })
            .collect();

        !digits.iter().zip(digits.iter().rev()).any(|(f, b)| f != b)
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(132323));
}
