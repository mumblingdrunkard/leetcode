// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/smallest-string-with-swaps/

struct Solution {}

fn alphabet_to_usize(c: u8) -> usize {
    (c - b'a') as usize
}

fn usize_to_alphabet(i: usize) -> u8 {
    i as u8 + b'a'
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        use std::collections::HashMap;
        #[inline]
        fn parent(i: usize, v: &mut [usize]) -> (usize, usize) {
            if i == v[i] {
                return (i, 0);
            }
            let (p, h) = parent(v[i], v);
            v[i] = p;
            return (p, h + 1);
        }

        let s = s.bytes().collect::<Vec<_>>();
        let mut p = vec![0; s.len()];
        for i in 0..s.len() {
            p[i] = i;
        }

        for pair in pairs {
            let (a, b) = (pair[0] as usize, pair[1] as usize);
            let (a, ha) = parent(a, &mut p);
            let (b, hb) = parent(b, &mut p);

            if a == b {
                continue;
            }

            if ha < hb {
                p[a] = p[b];
            } else {
                p[b] = p[a];
            }
        }

        let mut map = HashMap::new();

        for i in 0..s.len() {
            let (s, _) = parent(i, &mut p);
            map.entry(s).or_insert(vec![]).push(i);
        }

        let mut r = vec![b' '; s.len()];
        for set in map.values_mut() {
            set.sort();
            let mut characters =
                set.iter()
                    .map(|&i| alphabet_to_usize(s[i]))
                    .fold([0; 26], |mut a, c| {
                        a[c] += 1;
                        a
                    });

            let mut j = 0;
            for &mut i in set {
                while characters[j] < 1 {
                    j += 1;
                }
                characters[j] -= 1;
                r[i] = usize_to_alphabet(j);
            }
        }

        unsafe { String::from_utf8_unchecked(r) }
    }
}

fn main() {
    println!(
        "{}",
        Solution::smallest_string_with_swaps(
            "dcab".to_string(),
            vec![vec![0, 3], vec![1, 2], vec![0, 2]]
        )
    );
}
