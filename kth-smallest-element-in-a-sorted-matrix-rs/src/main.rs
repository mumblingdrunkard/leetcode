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
// https://leetcode.com/problems/kth-smallest-number-in-a-sorted-matrix/

struct Solution {}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut max = *matrix.last().unwrap().last().unwrap();
        let mut min = *matrix.first().unwrap().first().unwrap();
        let k = k as usize;

        while min <= max {
            let mid = min + (max - min)/2;
            let rank = Self::rank(&matrix, mid);
            if rank < k {
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }

        min
    }

    pub fn rank(matrix: &Vec<Vec<i32>>, a: i32) -> usize {
        let mut rank = 0;
        for i in 0..matrix.len() {
            let mut j = matrix.len();
            while j > 0 && matrix[i][j - 1] > a {
                j -= 1;
            }
            rank += j;
        }
        rank
    }
}

fn main() {
    println!(
        "{}",
        Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8)
    );
}
