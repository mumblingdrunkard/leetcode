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
// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/

struct Solution {}

impl Solution {
    pub fn max_sum_submatrix(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let target = k;
        fn subsum(from: (usize, usize), to: (usize, usize), m: &Vec<Vec<i32>>) -> i32 {
            m[to.0][to.1] - m[from.0 - 1][to.1] - m[to.0][from.1 - 1] + m[from.0 - 1][from.1 - 1]
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        // transform matrix to prefix sum over rows
        for r in 0..m {
            for c in 1..n {
                matrix[r][c] += matrix[r][c - 1];
            }
        }

        // transform matrix to prefix sum over submatrices
        // matrix[i][j] gives the sum of submatrix from (0, 0) to (i, j)
        for r in 1..m {
            for c in 0..n {
                matrix[r][c] += matrix[r - 1][c];
            }
        }

        let mut new_matrix = vec![vec![0; n + 1]];
        for mut r in matrix {
            r.insert(0, 0);
            new_matrix.push(r);
        }

        let mut max = i32::MIN;
        // check all the sums
        for i in 1..m + 1 {
            for j in i..m + 1 {
                for k in 1..n + 1 {
                    for l in k..n + 1 {
                        let subs = subsum((i, k), (j, l), &new_matrix);
                        if subs <= target {
                            max = std::cmp::max(subs, max);
                        }
                    }
                }
            }
        }

        max
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2)
    );
}
