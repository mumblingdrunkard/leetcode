// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/find-and-replace-pattern/

struct Solution {}

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .into_iter()
            .filter(|w| {
                use std::collections::HashMap;
                let mut map_uv = HashMap::new();
                let mut map_vu = HashMap::new();
                w.chars().zip(pattern.chars()).all(|(u, v)| {
                    match (map_uv.insert(u, v), map_vu.insert(v, u)) {
                        (Some(y), Some(x)) => x == u && y == v, // mapping exists and matches
                        (None, None) => true,                   // mapping does not already exist
                        _ => false,                             // mapping should exist in both
                    }
                })
            })
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_and_replace_pattern(
            vec![
                "abc".to_string(),
                "deq".to_string(),
                "mee".to_string(),
                "aqq".to_string(),
                "dkd".to_string(),
                "ccc".to_string()
            ],
            "abb".to_string()
        )
    );
}
