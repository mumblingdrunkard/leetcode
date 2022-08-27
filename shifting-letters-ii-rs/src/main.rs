// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/shifting-letters-ii/

struct Solution {}

fn alphabet_as_i32(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32,
        _ => panic!("Character was not a lowercase English letter"),
    }
}

fn i32_as_alphabet(n: i32) -> char {
    match n {
        0..=25 => ('a' as i32 + n) as u8 as char,
        _ => panic!(
            "Number was not in the correct range to be converted to a lowercase English letter"
        ),
    }
}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.chars().map(alphabet_as_i32).collect::<Vec<_>>();
        let mut v = vec![0; s.len() + 1];

        for shift in shifts {
            let (l, r, dir) = (shift[0] as usize, shift[1] as usize, shift[2]);
            let dir = match dir {
                0 => -1,
                d => d,
            };

            v[l] += dir;
            v[r + 1] -= dir;
        }

        let mut cumulative = 0;
        for (i, c) in s.iter_mut().enumerate() {
            cumulative += v[i];
            *c += cumulative;
            *c %= 26; // may be negative
            *c += 26;
            *c %= 26; // guaranteed positive
        }

        s.into_iter().map(i32_as_alphabet).collect()
    }
}

fn main() {
    println!(
        "{}",
        Solution::shifting_letters(
            "abc".to_string(),
            vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
        )
    );

    println!(
        "{}",
        Solution::shifting_letters("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]])
    );
}
