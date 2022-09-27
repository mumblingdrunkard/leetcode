// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/push-dominoes/

struct Solution {}

macro_rules! continue_if {
    ($res:expr) => {
        if $res {
            continue;
        }
    };
}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        use std::collections::VecDeque;
        let len = dominoes.len();
        let mut q = VecDeque::new();
        let mut dominoes = dominoes.into_bytes();

        q.extend(
            dominoes
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, d)| *d != b'.')
                .map(|(i, d)| (i, d, 0)),
        );

        let mut ages = vec![usize::MAX; len];

        while let Some((i, d, a)) = q.pop_front() {
            continue_if!(ages[i] < a);

            if ages[i] == a {
                dominoes[i] = b'.';
            } else {
                ages[i] = a;
                dominoes[i] = d;
            }

            match d {
                b'L' if i > 0 => q.push_back((i - 1, d, a + 1)),
                b'R' if i < len - 1 => q.push_back((i + 1, d, a + 1)),
                _ => {}
            }
        }

        String::from_utf8(dominoes).unwrap()
    }
}

fn main() {
    println!("{}", Solution::push_dominoes(".L.R...LR..L..".to_string()));
}
