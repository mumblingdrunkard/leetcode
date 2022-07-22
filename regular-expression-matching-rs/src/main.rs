// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/regular-expression-matching/

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // matches[i][j] = p[j..] matches s[i..]
        let mut matches = vec![vec![false; p.len() + 1]; s.len() + 1];
        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();
        matches[s.len()][p.len()] = true;

        // deal with stars at the end of pattern, matching the end of the string
        for (j, _) in p.iter().enumerate().rev() {
            matches[s.len()][j] = j + 1 < p.len() && p[j + 1] == '*' && matches[s.len()][j + 2];
        }

        for i in (0..s.len()).rev() {
            // for each character, backwards in the string
            for (j, b) in p.iter().enumerate().rev() {
                // for each pattern, in reverse
                let check = match b {
                        '.' => true,
                        &c => c == s[i],
                    };

                matches[i][j] = if j + 1 < p.len() && p[j + 1] == '*' {
                    // rest of pattern matches, skipping this char, OR
                    // rest of string matches, repeating this pattern
                    matches[i][j + 2] || check && matches[i + 1][j]
                } else {
                    // check that rest of pattern matches rest of string
                    check && matches[i + 1][j + 1]
                };
            }
        }

        matches[0][0]
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_match("aa".to_string(), "a*aa".to_string())
    );
}
