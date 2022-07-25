// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/maximum-units-on-a-truck/

struct Solution {}

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut boxes: Vec<_> = box_types.iter().map(|r| (r[1], r[0])).collect();
        boxes.sort();
        boxes
            .iter()
            .rev()
            .scan(0, |total_boxes, (units, boxes)| {
                let to_add = std::cmp::min(truck_size - *total_boxes, *boxes);
                *total_boxes += to_add;
                (*total_boxes <= truck_size).then(|| to_add * units)
            })
            .sum()
    }
}

fn main() {
    println!("{}", Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4));
}
