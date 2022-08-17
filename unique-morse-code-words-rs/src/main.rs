// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/unique-morse-code-words/

struct Solution {}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let map = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .zip(
                [
                    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-",
                    ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-",
                    ".--", "-..-", "-.--", "--..",
                ]
                .iter(),
            )
            .collect::<HashMap<_, _>>();

        words
            .iter()
            .fold(HashSet::new(), |mut set, w| {
                // map each word to its corresponding morse-code
                let morse = w
                    .chars()
                    .map(|c| **map.get(&c).unwrap())
                    .collect::<String>();
                // accumulate the morse-code in the set
                set.insert(morse);
                set
            })
            .len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string(),
        ])
    );
}
