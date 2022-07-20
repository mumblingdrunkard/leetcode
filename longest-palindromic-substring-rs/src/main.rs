// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/longest-palindromic-substring/

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn with_start(chars: &[char], start: (usize, usize)) -> (usize, usize) {
            let (mut i, mut j) = start;

            if chars[i] != chars[j] {
                return (0, 0)
            }

            loop {
                if i == 0 || j == chars.len() - 1 {
                    break;
                }

                if chars[i - 1] == chars[j + 1] {
                    i -= 1;
                    j += 1;
                } else {
                    break;
                }
            }
            (i, j)
        }

        let chars: Vec<_> = s.chars().collect();

        let (i1, j1) = (0..chars.len())
            .map(|i| with_start(&chars, (i, i)))
            .max_by(|(a1, a2), (b1, b2)| (a2 - a1).cmp(&(b2 - b1)))
            .unwrap_or((0, 0));

        let (i2, j2) = (0..chars.len() - 1)
            .map(|i| with_start(&chars, (i, i + 1)))
            .max_by(|(a1, a2), (b1, b2)| (a2 - a1).cmp(&(b2 - b1)))
            .unwrap_or((0, 0));

        if (j1 - i1) > (j2 - i2) {
            chars[i1..j1 + 1].iter().collect()
        } else {
            chars[i2..j2 + 1].iter().collect()
        }
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("cbbd".to_string()));
}
