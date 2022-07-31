// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/range-sum-query-mutable/

struct NumArray {
    fenwick: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut na = Self {
            fenwick: vec![0; nums.len() + 1],
        };
        for (i, n) in nums.into_iter().enumerate() {
            na.add(i + 1, n);
        }
        na
    }

    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize + 1;
        self.set(index, val);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize + 1;
        let right = right as usize + 1;
        self.get(right) - self.get(left - 1)
    }

    fn get(&self, i: usize) -> i32 {
        let (mut sum, mut i) = (0, i);
        while i > 0 {
            sum += self.fenwick[i]; // accumulate the sum
            i -= i & i.overflowing_neg().0; // parent element
        }
        sum
    }

    fn add(&mut self, mut i: usize, n: i32) {
        while i < self.fenwick.len() {
            self.fenwick[i] += n;
            i += i & i.overflowing_neg().0; // next element
        }
    }

    fn set(&mut self, i: usize, n: i32) {
        let diff = n - self.get(i) + self.get(i - 1);
        if diff == 0 {
            return;
        }
        self.add(i, diff);
    }
}

fn main() {
    let mut na = NumArray::new(vec![1, 3, 5]);
    println!("{}", na.sum_range(0, 2));
    na.update(1, 2);
    println!("{}", na.sum_range(0, 2));
}
