// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/largest-perimeter-triangle/

struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        let mut h = nums.into_iter().collect::<BinaryHeap<_>>();
        let mut c = h.pop().unwrap();
        let mut b = h.pop().unwrap();
        let mut a = h.pop().unwrap();

        while a + b <= c {
            c = b;
            b = a;
            if let Some(n) = h.pop() {
                a = n;
            } else {
                return 0;
            }
        }

        a + b + c
    }
}

fn main() {
    println!("{}", Solution::largest_perimeter(vec![2, 1, 2]));
    println!("{}", Solution::largest_perimeter(vec![1, 2, 1]));
}
