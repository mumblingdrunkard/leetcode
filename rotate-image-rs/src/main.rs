// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/rotate-image/

struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn rotate_old(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            let w = n - 2 * i;
            for j in i..n - i - 1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[i + i + w - 1 - j][i];
                matrix[i + i + w - 1 - j][i] = matrix[i + w - 1][i + i + w - 1 - j];
                matrix[i + w - 1][i + i + w - 1 - j] = matrix[j][i + w - 1];
                matrix[j][i + w - 1] = tmp;
            }
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            matrix.swap(i, n - i - 1);
        }

        for i in 1..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}

fn main() {
    let mut mat = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];

    println!("Original:");
    for r in &mat {
        println!("{r:?}");
    }

    Solution::rotate(&mut mat);

    println!("Rotated:");
    for r in &mat {
        println!("{r:?}");
    }

    let mut mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    println!("Original:");
    for r in &mat {
        println!("{r:?}");
    }

    Solution::rotate(&mut mat);

    println!("Rotated:");
    for r in &mat {
        println!("{r:?}");
    }
}
