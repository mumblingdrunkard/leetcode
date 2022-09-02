// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/pacific-atlantic-water-flow/

struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let (m, n) = (heights.len(), heights[0].len());
        const PACIFIC: u8 = 0b01;
        const ATLANTIC: u8 = 0b10;

        let mut reaches = vec![vec![0u8; n]; m];
        let mut q = VecDeque::new();
        for i in 0..n {
            reaches[0][i] |= PACIFIC;
            reaches[m - 1][i] |= ATLANTIC;
            q.push_back((0, i));
            q.push_back((m - 1, i));
        }

        for i in 0..m {
            reaches[i][0] |= PACIFIC;
            reaches[i][n - 1] |= ATLANTIC;
            q.push_back((i, 0));
            q.push_back((i, n - 1));
        }

        while let Some((r, c)) = q.pop_front() {
            let reach = reaches[r][c];
            let height = heights[r][c];
            if r > 0
                && height <= heights[r - 1][c]
                && reach | reaches[r - 1][c] != reaches[r - 1][c]
            {
                reaches[r - 1][c] |= reach;
                q.push_back((r - 1, c));
            }

            if r + 1 < m
                && height <= heights[r + 1][c]
                && reach | reaches[r + 1][c] != reaches[r + 1][c]
            {
                reaches[r + 1][c] |= reach;
                q.push_back((r + 1, c));
            }

            if c > 0
                && height <= heights[r][c - 1]
                && reach | reaches[r][c - 1] != reaches[r][c - 1]
            {
                reaches[r][c - 1] |= reach;
                q.push_back((r, c - 1));
            }

            if c + 1 < n
                && height <= heights[r][c + 1]
                && reach | reaches[r][c + 1] != reaches[r][c + 1]
            {
                reaches[r][c + 1] |= reach;
                q.push_back((r, c + 1));
            }
        }

        (0..m)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .filter_map(|(r, c)| (reaches[r][c] == 0b11).then(|| vec![r as i32, c as i32]))
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ])
    );
}
