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
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // idea: sliding window and track where each character was seen, track the largest window
        // possible
        use std::collections::HashMap;

        let mut i = 0; // the index of the leftmost character in the current substring
        let mut max = 0;

        // can optimise this to a vec and map letters to indices instead
        let mut seen = HashMap::new(); // track index of each character in the current substring

        let chars: Vec<_> = s.chars().collect();

        for (j, c) in chars.iter().enumerate() {
            if let Some(&idx) = seen.get(&c) {
                while i <= idx {
                    seen.remove(&chars[i]);
                    i += 1;
                }
            }

            seen.insert(c, j);
            max = std::cmp::max(max, j - i + 1);
        }

        max as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
}
