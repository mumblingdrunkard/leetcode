// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/find-all-anagrams-in-a-string/

struct Solution {}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }

        let mut counts = p.as_bytes().iter().fold([0; 26], |mut counts, c| {
            counts[(c - b'a') as usize] += 1;
            counts
        });

        let mut unmatched = counts.iter().filter(|&&c| c != 0).count();
        let s = s.as_bytes();

        let mut i = 0;
        while i < p.len() {
            let c = (s[i] - b'a') as usize;
            if counts[c] == 0 {
                unmatched += 1;
            }
            counts[c] -= 1;
            if counts[c] == 0 {
                unmatched -= 1;
            }
            i += 1;
        }

        let mut res = vec![];

        if unmatched == 0 {
            res.push(0);
        }

        let mut j = 0;
        while i < s.len() {
            let removed = (s[j] - b'a') as usize;
            let added = (s[i] - b'a') as usize;
            if counts[removed] == -1 {
                unmatched -= 1;
            } else if counts[removed] == 0 {
                unmatched += 1;
            }
            counts[removed] += 1;

            if counts[added] == 1 {
                unmatched -= 1;
            } else if counts[added] == 0 {
                unmatched += 1;
            }
            counts[added] -= 1;

            j += 1;
            i += 1;

            if unmatched == 0 {
                res.push(j as i32);
            }
        }

        res
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_anagrams("abcbacba".to_string(), "abc".to_string())
    );
}
