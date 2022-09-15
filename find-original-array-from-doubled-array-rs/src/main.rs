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
// https://leetcode.com/problems/find-original-array-from-doubled-array/

struct Solution {}

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut um = HashMap::new();
        let mut orig = vec![];

        if changed.len() % 2 != 0 {
            return vec![];
        }

        changed.sort();

        for c in changed {
            if c % 2 == 0 {
                if let e @ 1.. = um.entry(c / 2).or_insert(0) {
                    *e -= 1;
                    orig.push(c / 2);
                    continue;
                }
            }

            if let e @ 1.. = um.entry(c * 2).or_insert(0) {
                *e -= 1;
                orig.push(c);
                continue;
            }

            *um.entry(c).or_insert(0) += 1;
        }

        if um.iter().all(|(_, &v)| v == 0) {
            orig
        } else {
            vec![]
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_original_array(vec![1, 3, 4, 2, 6, 8])
    );
}
