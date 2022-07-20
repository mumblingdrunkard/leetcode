// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/pascals-triangle/

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        { 0..num_rows }
            .scan(vec![1], |current, _| {
                let mut previous = [0]
                    .iter()
                    .chain(current.iter())
                    .zip(current.iter().chain([0].iter()))
                    .map(|(l, r)| l + r)
                    .collect();
                std::mem::swap(&mut previous, current);
                Some(previous)
            })
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::generate(30));
}
