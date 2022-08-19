// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/reduce-array-size-to-the-half/

struct Solution {}

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        // store this for later
        let n = arr.len();

        // count frequencies
        let map = arr
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut map, n| {
                *map.entry(n).or_insert(0) += 1;
                map
            });

        // extract and sort by frequency (don't worry about keys)
        let mut freq = map.iter().map(|(_, v)| *v).collect::<Vec<_>>();
        freq.sort();

        // remove numbers by decreasing frequency until at least half are removed, tracking the
        // size of the set of removed numbers
        let mut set_size = 0;
        let mut removed = 0;
        for &f in freq.iter().rev() {
            removed += f;
            set_size += 1;
            if removed * 2 >= n {
                break;
            }
        }

        set_size
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7])
    );

    println!("{}", Solution::min_set_size(vec![7, 7, 7, 7, 7]));
}
