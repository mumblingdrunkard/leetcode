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
// https://leetcode.com/problems/largest-number-after-digit-swaps-by-parity/

struct Solution {}

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        // extract the digits
        let digits = (0..).scan(num, |rem, _| {
            (*rem > 0).then(|| {
                let d = *rem % 10;
                *rem /= 10;
                d
            })
        });

        // collect digits into containers and track which indices are even or odd
        let (mut evens, mut odds, is_even) = digits.fold(
            (vec![], vec![], vec![]),
            |(mut evens, mut odds, mut is_even), d| {
                if d % 2 == 0 {
                    evens.push(d);
                    is_even.push(true);
                } else {
                    odds.push(d);
                    is_even.push(false);
                }
                (evens, odds, is_even)
            },
        );

        // sort evens and odds
        evens.sort();
        odds.sort();

        is_even
            // for each index
            .into_iter()
            // in reverse
            .rev()
            // append the largest unused even or odd number depending on the parity of the index
            .fold(0, |result, is_even| match is_even {
                // unwrapping here is fine because there will always be enough numbers
                true => result * 10 + evens.pop().unwrap(),
                _ => result * 10 + odds.pop().unwrap(),
            })
    }
}

fn main() {
    println!("{}", Solution::largest_integer(1234));
}
