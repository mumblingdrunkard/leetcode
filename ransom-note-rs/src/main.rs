// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/ransom-note/

struct Solution {}

fn alphabet_to_usize(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize,
        _ => panic!("Not a lowercase English letter"),
    }
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut characters = [0; 26];
        for c in magazine.chars().map(alphabet_to_usize) {
            characters[c] += 1;
        }

        for c in ransom_note.chars().map(alphabet_to_usize) {
            characters[c] -= 1;
        }

        characters.iter().all(|&n| n >= 0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_construct("aa".to_string(), "aab".to_string())
    );
}
