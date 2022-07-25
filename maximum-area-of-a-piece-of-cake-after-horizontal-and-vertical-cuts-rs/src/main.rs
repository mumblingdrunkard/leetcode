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
// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/

struct Solution {}

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts;
        let mut vertical_cuts = vertical_cuts;
        horizontal_cuts.push(h);
        vertical_cuts.push(w);
        horizontal_cuts.sort();
        vertical_cuts.sort();

        // the max distance between horisontal cuts
        let w = horizontal_cuts
            .iter()
            .fold((0, 0), |(previous, max), &current| {
                (current, std::cmp::max(max, current - previous))
            })
            .1 as i64;

        // the max distance between vertical cuts
        let h = vertical_cuts
            .iter()
            .fold((0, 0), |(previous, max), &current| {
                (current, std::cmp::max(max, current - previous))
            })
            .1 as i64;

        // calculate area mod 10⁹+7 and cast to i32
        ((h * w) % 1000000007) as i32
    }
}

fn main() {
    println!("{}", Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]));
}
