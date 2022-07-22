// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/wildcard-matching/

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // pretty much same solution as regular-expression-matching
        let mut matches = vec![vec![false; p.len() + 1]; s.len() + 1];
        matches[s.len()][p.len()] = true;

        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();

        for (j, &b) in p.iter().enumerate().rev() {
            matches[s.len()][j] = b == '*' && matches[s.len()][j + 1];
        }

        for (i, &a) in s.iter().enumerate().rev() {
            for (j, &b) in p.iter().enumerate().rev() {
                let check = match b {
                    '?' => true,
                    c => c == a,
                };

                matches[i][j] = if b == '*' {
                    // rest of string matches; repeating this *, OR
                    // rest of pattern matches; skipping this *,
                    matches[i + 1][j] || matches[i][j + 1]
                } else {
                    check && matches[i + 1][j + 1]
                };
            }
        }

        matches[0][0]
    }
}

fn main() {
    println!("{}", Solution::is_match("aa".to_string(), "*??".to_string()));
}
