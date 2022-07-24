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
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits == "" {
            return vec![];
        }

        let chars: Vec<_> = digits.chars().collect();

        chars.iter().rev().fold(vec!["".to_string()], |current, key| {
            current
                .into_iter()
                .flat_map(|rest| {
                    Solution::key_to_str(key)
                        .chars()
                        .map(move |c| format!("{}{}", c, rest))
                })
                .collect()
        })
    }

    fn key_to_str(key: &char) -> &'static str {
        match key {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => "",
        }
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("2662437".to_string()));
}
