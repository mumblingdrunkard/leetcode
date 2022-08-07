// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/count-vowels-permutation/

struct Solution {}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        // 0 -> 'a'
        // 1 -> 'e'
        // 2 -> 'i'
        // 3 -> 'o'
        // 4 -> 'u'
        // =>
        // 0 can follow { 1, 2, 4 }
        // 1 can follow { 0, 2 }
        // 2 can follow { 1, 3 }
        // 3 can follow { 2 }
        // 4 can follow { 2, 3 }
        const M: i32 = 1000000007;
        let n = n as usize;

        let p = (1..n).fold((1, 1, 1, 1, 1), |p, _| {
            (
                ((p.1 + p.2) % M + p.4) % M,
                (p.0 + p.2) % M,
                (p.1 + p.3) % M,
                p.2,
                (p.2 + p.3) % M,
            )
        });

        // this is ducking fugly
        (((p.0 + p.1) % M + (p.2 + p.3) % M) % M + p.4) % M
    }
}

fn main() {
    println!("{}", Solution::count_vowel_permutation(5));
}
