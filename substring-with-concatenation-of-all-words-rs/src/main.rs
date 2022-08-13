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
// https://leetcode.com/problems/substring-with-concatenation-of-all-words/

struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result = vec![];
        let s = s.chars().collect::<Vec<_>>();
        let wlen = words[0].len();
        let nwords = words.len();
        let map = words
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut map, w| {
                *map.entry(w.chars().collect::<Vec<_>>()).or_insert(0) += 1;
                map
            });

        for i in 0..wlen {
            let mut submap = map.clone();
            let mut found = 0;
            let (mut start, mut end) = (i, i);
            while end + wlen <= s.len() {
                while found < nwords && end + wlen <= s.len() {
                    if let Some(e) = submap.get_mut(&s[end..end + wlen]) {
                        *e -= 1;
                        found += 1;
                        end = end + wlen;
                        println!("{}", s[start..end].iter().collect::<String>());
                    } else {
                        while start < end {
                            if let Some(e) = submap.get_mut(&s[start..start + wlen]) {
                                *e += 1;
                            }
                            start = start + wlen;
                            found -= 1;
                        }
                        start = end + wlen;
                        end = end + wlen;
                    }
                }

                if submap.iter().all(|(_, &v)| v == 0) {
                    result.push(start as i32);
                }

                if end + wlen > s.len() {
                    break;
                }

                if let Some(e) = submap.get_mut(&s[start..start + wlen]) {
                    *e += 1;
                    start = start + wlen;
                    found -= 1;
                }
            }
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "good".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        )
    );
}
