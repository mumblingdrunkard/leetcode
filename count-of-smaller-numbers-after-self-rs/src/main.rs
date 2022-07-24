// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/count-of-smaller-numbers-after-self/

struct Solution {}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        fn get_next(i: usize) -> usize {
            i + (i & i.overflowing_neg().0)
        }

        fn get_parent(i: usize) -> usize {
            i - (i & i.overflowing_neg().0)
        }

        // for each number,
        nums.iter()
            // ... in reverse
            .rev()
            // track numbers with a fenwick tree
            .scan(vec![0; 20002], |fenwick, n| {
                // offset n such that all values are strictly larger than 0
                let n = (n + 10001) as usize;

                // update the fenwick tree for all elements larger than n
                let mut i = n + 1;
                while i < fenwick.len() {
                    fenwick[i] += 1; // adding one
                    i = get_next(i);
                }

                // calculate the value for n
                let (mut sum, mut i) = (0, n);
                while i > 0 {
                    sum += fenwick[i];
                    i = get_parent(i);
                }

                // yield the sum
                Some(sum)
            })
            // collect all the sums
            .collect::<Vec<i32>>()
            // then reverse the vector again
            .into_iter()
            .rev()
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::count_smaller(vec![5, 2, 6, 1]));
}