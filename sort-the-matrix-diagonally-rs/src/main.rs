// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/sort-the-matrix-diagonally/

struct Solution {}

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut diag = vec![vec![]; m + n - 1];
        for (i, r) in mat.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                diag[j + m - i - 1].push(c);
            }
        }

        for d in &mut diag {
            d.sort();
        }

        let mut o = (m - 1, 0);
        for (_, d) in diag.iter().enumerate() {
            for (j, &c) in d.iter().enumerate() {
                mat[o.0 + j][o.1 + j] = c;
            }

            if o.0 == 0 {
                o.1 += 1;
            } else {
                o.0 -= 1;
            }
        }

        mat
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::diagonal_sort(vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50]
        ])
    );
}
