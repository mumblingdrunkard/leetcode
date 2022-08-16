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
// https://leetcode.com/problems/special-positions-in-a-binary-matrix/

struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let rsum = mat
            .iter()
            .map(|r| r.iter().sum::<i32>())
            .collect::<Vec<_>>();

        let csum = (0..mat[0].len())
            .map(|ci| mat.iter().map(|r| r[ci]).sum::<i32>())
            .collect::<Vec<_>>();

        let mut special = 0;
        for (r, row) in mat.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 && rsum[r] == 1 && csum[c] == 1 {
                    special += 1;
                }
            }
        }

        special
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_special(vec![
            vec![0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0, 0]
        ])
    );
}
