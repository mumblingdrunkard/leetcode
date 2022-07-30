// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/word-subsets/

struct Solution {}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        // construct the universal word by
        let universal = words2.iter().fold(HashMap::new(), |mut u, s| {
            // taking the count of each character of each word
            let v = s.chars().fold(HashMap::new(), |mut v, c| {
                let count = v.entry(c).or_insert(0usize);
                *count += 1;
                v
            });

            // and finding the maximum occurence of each character
            for (c, count) in v {
                let current = u.entry(c).or_insert(0);
                *current = std::cmp::max(*current, count);
            }

            u
        });

        // for each word
        words1
            .into_iter()
            .filter(|s| {
                // count occurences of each character in the word
                let v = s.chars().fold(HashMap::new(), |mut v, c| {
                    let count = v.entry(c).or_insert(0usize);
                    *count += 1;
                    v
                });

                // ensure that the word has at least the characters in the universal word
                universal
                    .iter()
                    .all(|(c, count)| v.get(c).unwrap_or(&0) >= count)
            })
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string(),
            ],
            vec!["e".to_string(), "o".to_string(),]
        )
    );
}
