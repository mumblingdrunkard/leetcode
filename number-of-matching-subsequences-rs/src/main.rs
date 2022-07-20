// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/number-of-matching-subsequences/

struct Solution {}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let indices = s.chars().enumerate().fold(
            HashMap::new(),
            |mut map: HashMap<char, Vec<usize>>, (i, c)| {
                if let Some(v) = map.get_mut(&c) {
                    v.push(i);
                } else {
                    map.insert(c, vec![i]);
                };
                map
            },
        );

        words
            .iter()
            .map(|w| {
                let (_, valid) = w.chars().fold((0, true), |(idx, valid), c| {
                    if !valid {
                        return (0, false);
                    }

                    match indices.get(&c) {
                        Some(v) => match v.binary_search(&idx) {
                            Ok(i) => (v[i] + 1, valid),
                            Err(i) => (v.get(i).unwrap_or(&0) + 1, i < v.len()),
                        },
                        None => (0, false),
                    }
                });

                match valid {
                    true => 1,
                    _ => 0,
                }
            })
            .sum()
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_matching_subseq(
            "abcde".to_string(),
            vec![
                "a".to_string(),
                "bb".to_string(),
                "acd".to_string(),
                "ace".to_string()
            ]
        )
    );
}
