// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/n-queens/

struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        Self::n_queens(&vec![vec![0; n]; n], 0)
    }

    fn n_queens(board: &Vec<Vec<u8>>, row: usize) -> Vec<Vec<String>> {
        // 0 = unblocked position
        // 1 = blocked position
        // 2 = queen
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
            // fill the diagonals and the column of the proceeding rows
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
    println!("{:?}", Solution::solve_n_queens(9));
}
