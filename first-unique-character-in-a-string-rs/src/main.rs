// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/first-unique-character-in-a-string/

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[Self::map(c)] += 1;
        }

        s.chars()
            .position(|c| count[Self::map(c)] == 1)
            .map_or(-1, |v| v as i32)
    }

    fn map(c: char) -> usize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            'i' => 8,
            'j' => 9,
            'k' => 10,
            'l' => 11,
            'm' => 12,
            'n' => 13,
            'o' => 14,
            'p' => 15,
            'q' => 16,
            'r' => 17,
            's' => 18,
            't' => 19,
            'u' => 20,
            'v' => 21,
            'w' => 22,
            'x' => 23,
            'y' => 24,
            'z' => 25,
            _ => panic!(),
        }
    }
}

fn main() {
    println!("{}", Solution::first_uniq_char("loveleetcode".to_string()));
}
