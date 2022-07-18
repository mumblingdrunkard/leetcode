// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/trapping-rain-water-ii/

struct Solution {}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        use std::cmp::Reverse;

        // set water level to i32::MAX in all cells, also tracks whether the cell contains water or
        // not
        let mut level: Vec<Vec<_>> = height_map
            .iter()
            .map(|r| r.iter().map(|_| (i32::MAX, true)).collect())
            .collect();

        // push all edges to queue to drain out water, ordering from lowest height to tallest
        // height (using Reverse).
        // This ordering ensures the fastest draining, although it should not affect the
        // correctness of the algorithm.
        let mut q = std::collections::BinaryHeap::new();

        // top row
        for (i, h) in height_map.first().unwrap().iter().enumerate() {
            q.push(Reverse((*h, 0, i)));
        }

        // bottom row
        for (i, h) in height_map.last().unwrap().iter().enumerate() {
            q.push(Reverse((*h, height_map.len() - 1, i)));
        }

        // columns
        for (i, r) in height_map.iter().enumerate() {
            let h_left = *r.first().unwrap();
            let h_right = *r.last().unwrap();
            q.push(Reverse((h_left, i, 0)));
            q.push(Reverse((h_right, i, r.len() - 1)));
        }

        // the above sequence of pushes will push each of the four corners twice.
        // This doesn't really matter

        // while there are still cells in the queue which should be drained
        while !q.is_empty() {
            let (h0, r, c) = q.pop().unwrap().0;
            let (h1, _) = level[r][c];

            // current level is already lower than the drain level
            if h0 >= h1 {
                continue;
            }

            // drain to max of drain level or height in height_map
            let new_level = max(height_map[r][c], h0);
            level[r][c] = (new_level, new_level > height_map[r][c]);
            let h = level[r][c].0;

            // check if any neighbours have a higher level than this cells level and that they
            // contain water, if so push them to the queue with this cells new level.
            if r > 0 {
                if let (h1, true) = level[r - 1][c] {
                    if h1 > h {
                        q.push(Reverse((h, r - 1, c)));
                    }
                }
            }

            if r + 1 < height_map.len() {
                if let (h1, true) = level[r + 1][c] {
                    if h1 > h {
                        q.push(Reverse((h, r + 1, c)));
                    }
                }
            }

            if c > 0 {
                if let (h1, true) = level[r][c - 1] {
                    if h1 > h {
                        q.push(Reverse((h, r, c - 1)));
                    }
                }
            }

            if c + 1 < height_map.first().unwrap().len() {
                if let (h1, true) = level[r][c + 1] {
                    if h1 > h {
                        q.push(Reverse((h, r, c + 1)));
                    }
                }
            }
        }

        // take level - height_map and sum up the cells
        height_map
            .iter()
            .zip(level.iter())
            .map(|(h, l)| {
                h.iter()
                    .zip(l.iter())
                    .map(|(h, (l, _))| l - h)
                    .sum::<i32>()
            })
            .sum()
    }
}

fn main() {
    #[rustfmt::skip]
    println!(
        "{}",
        Solution::trap_rain_water(vec![
            vec![12, 13,  1, 12],
            vec![13,  4, 13, 12],
            vec![13,  8, 10, 12],
            vec![12, 13, 12, 12],
            vec![13, 13, 13, 13],
        ])
    );
}

// edge case:
// 12  13   1  12
// 13   4  13  12
// 13   8  10  12
// 12  13  12  12
// 13  13  13  13
