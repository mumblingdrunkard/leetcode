// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/n-queens-ii/

struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        // *this is a bad solution, using the same solution as the original n-queens problem*
        let n = n as usize;
        Self::n_queens(&vec![vec![0; n]; n], 0).len() as i32
    }

    fn n_queens(board: &Vec<Vec<u8>>, row: usize) -> Vec<Vec<String>> {
        if row == board.len() {
            return vec![board
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|&u| (u == 2).then(|| 'Q').unwrap_or('.'))
                        .collect()
                })
                .collect()];
        }

        // try to place a queen on the row
        let open_slots = board[row]
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == 0)
            .map(|(i, _)| i);

        let mut result = vec![];
        for col in open_slots {
            let mut b = board.clone();
            b[row][col] = 2;
            for i in 1..b.len() - row {
                if col >= i {
                    b[row + i][col - i] = 1;
                }

                if col + i < b[row].len() {
                    b[row + i][col + i] = 1;
                }

                b[row + i][col] = 1;
            }

            for res in Self::n_queens(&b, row + 1) {
                result.push(res);
            }
        }

        result
    }
}

fn main() {
    println!("{}", Solution::total_n_queens(9));
}
