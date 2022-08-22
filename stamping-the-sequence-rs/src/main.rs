// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/stamping-the-sequence/

struct Solution {}

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        use std::collections::VecDeque;
        // idea: use substrings of `stamp` to "shoot" holes in `target` until all letters are
        // replaced with holes.
        let stamp = stamp.chars().map(Self::alphabet_index).collect::<Vec<_>>();
        let mut target = target.chars().map(Self::alphabet_index).collect::<Vec<_>>();
        let mut stamps = vec![];

        let mut q = VecDeque::new();

        // search for initial stamps
        for i in 0..target.len() - stamp.len() + 1 {
            if Self::matches(&stamp, &target[i..]) {
                // put holes in target
                target[i..i + stamp.len()].iter_mut().for_each(|c| *c = 0);
                stamps.push(i);
                q.push_back(i);
            }
        }

        while let Some(edge) = q.pop_front() {
            if edge == 0 || edge == target.len() - 1 {
                continue;
            }

            let range = if edge < stamp.len() {
                0..edge
            } else {
                edge - stamp.len()..edge
            };

            for i in range {
                if Self::matches(&stamp, &target[i..]) {
                    // put holes in target
                    if target[i..i + stamp.len()].iter().all(|&t| t == 0) {
                        continue;
                    }
                    target[i..i + stamp.len()].iter_mut().for_each(|c| *c = 0);
                    stamps.push(i);
                    q.push_back(i);
                    break;
                }
            }

            let range = if edge + stamp.len() + stamp.len() > target.len() {
                edge + 1..target.len() - stamp.len() - 1
            } else {
                edge + 1..edge + stamp.len()
            };

            for i in range.rev() {
                if Self::matches(&stamp, &target[i..]) {
                    // put holes in target
                    if target[i..i + stamp.len()].iter().all(|&t| t == 0) {
                        continue;
                    }
                    target[i..i + stamp.len()].iter_mut().for_each(|c| *c = 0);
                    stamps.push(i);
                    q.push_back(i);
                    break;
                }
            }
        }

        if target.iter().any(|&k| k != 0) {
            return vec![];
        }

        stamps.into_iter().rev().map(|k| k as i32).collect()
    }

    fn matches(stamp: &[u8], target: &[u8]) -> bool {
        stamp
            .iter()
            .zip(target.iter())
            .all(|(&s, &t)| s == t || t == 0)
    }

    fn alphabet_index(c: char) -> u8 {
        match c {
            'a'..='z' => c as u8 - 'a' as u8 + 1,
            _ => panic!("Character was not a lowercase English letter!"),
        }
    }
}

fn main() {
    // println!(
    //     "{:?}",
    //     Solution::moves_to_stamp("abc".to_string(), "aabcbc".to_string())
    // );

    println!(
        "{:?}",
        Solution::moves_to_stamp("abc".to_string(), "ababc".to_string())
    );
}
