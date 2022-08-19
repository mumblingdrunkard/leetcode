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
// https://leetcode.com/problems/split-array-into-consecutive-subsequences/

struct Solution {}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let mut freq = nums.iter().fold(HashMap::new(), |mut map, &n| {
            *map.entry(n).or_insert(0) += 1;
            map
        });

        let mut nums = nums;
        nums.dedup();

        let mut tails = HashMap::new();

        for &n in &nums {
            let f = freq.get_mut(&n).unwrap();

            if *f < 0 {
                // we've over-indulged in numbers
                return false;
            }

            let t = tails.entry(n - 1).or_insert(0);

            if *f > *t {
                // if there are more n's than tails of n-1
                let diff = *f - *t;
                *t = 0; // there will be no tails left
                drop(t);
                *tails.entry(n).or_insert(0) += *f - diff; // we can create some tails with the old
                                                           // tails
                *f = 0; // all used up

                // the unmatched numbers are the start of sequences of [n, n+1, n+2)
                *freq.entry(n + 1).or_insert(0) -= diff;
                *freq.entry(n + 2).or_insert(0) -= diff;
                // the tail of these unmatched numbers will be n+2
                *tails.entry(n + 2).or_insert(0) += diff;
            } else {
                // create tails with the matched numbers
                *t -= *f;
                drop(t);
                *tails.entry(n).or_insert(0) += *f;
                *f = 0;
            }
        }

        freq.iter().all(|(_, &v)| v == 0)
    }
}

fn main() {
    println!("{}", Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    println!("{}", Solution::is_possible(vec![1, 2, 3, 5, 5, 6, 7]));
}
