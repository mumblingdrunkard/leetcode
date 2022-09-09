// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// This Source Code Form is “Incompatible With Secondary Licenses”, as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at
// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/

struct Solution {}

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let groups = properties.into_iter().fold(HashMap::new(), |mut m, v| {
            m.entry(v[0]).or_insert(vec![]).push(v[1]);
            m
        });

        let mut groups = groups.into_iter().collect::<Vec<_>>();
        groups.sort();

        // iterate in reverse
        let mut max = 0;
        let mut weak = 0;
        for (_, dv) in groups.into_iter().rev() {
            let local_max = *dv.iter().max().unwrap();
            for d in dv {
                if d < max {
                    weak += 1;
                }
            }
            max = std::cmp::max(local_max, max);
        }

        weak
    }
}

fn main() {
    println!(
        "{}",
        Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]])
    );

    println!(
        "{}",
        Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]])
    );

    println!(
        "{}",
        Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]])
    );
}
