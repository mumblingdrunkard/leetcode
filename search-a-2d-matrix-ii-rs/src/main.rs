// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/search-a-2d-matrix-ii/

struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix
            // for each row,
            .iter()
            // get only rows that can contain target
            .filter(|r| *r.first().unwrap() <= target && *r.last().unwrap() >= target)
            // binary search for target
            .any(|r| r.binary_search(&target).is_ok())
    }
}

fn main() {
    println!(
        "{}",
        Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        )
    );
}
