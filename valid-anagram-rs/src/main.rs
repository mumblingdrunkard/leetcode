// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/valid-anagram/

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // simple guard clause, ensures that we only need to check one way
        // otherwise, t might contain all of s and more, which would be false, but not caught by
        // the proceeding code
        if s.len() != t.len() {
            return false;
        }

        use std::collections::HashMap;
        // count occurences of each letter in t
        let mut tchars = t.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        // all letters in s can be matched to a letter in t
        s.chars().all(|c| {
            tchars
                // check that letter exists in t
                .get_mut(&c)
                // and_then check that there are still unmatched letters
                .and_then(|v| (*v > 0).then(|| *v -= 1)) // in which case we decrement the count
                // and reduce the Option returned from .then() to a boolean
                .is_some()
        })
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
    println!(
        "{}",
        Solution::is_anagram("car".to_string(), "rat".to_string())
    );
}
