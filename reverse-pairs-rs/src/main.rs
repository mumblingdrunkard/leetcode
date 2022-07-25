// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/reverse-pairs/

struct Solution {}

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        fn get_next(i: usize) -> usize {
            i + (i & i.overflowing_neg().0)
        }

        fn get_parent(i: usize) -> usize {
            i - (i & i.overflowing_neg().0)
        }

        let mut sorted = nums.clone();
        sorted.sort();
        sorted.dedup();
        let sorted: Vec<_> = sorted.into_iter().map(|x| x as i64).collect();

        // for each number,
        nums.iter()
            // ... in reverse
            .rev()
            // track numbers with a fenwick tree
            .scan(vec![0; sorted.len() + 2], |fenwick, &n| {
                // offset n such that all values are strictly larger than 0
                let doubled = match sorted.binary_search(&(n as i64 * 2 + 1)) {
                    Ok(index) => index,
                    Err(index) => index,
                } + 1;
                let n = sorted.binary_search(&(n as i64)).unwrap() + 1;

                // calculate the value for n
                let (mut sum, mut i) = (0, n);
                while i > 0 {
                    sum += fenwick[i];
                    i = get_parent(i);
                }

                // update the fenwick tree for all elements larger than n*2+1
                let mut i = doubled;
                while i < fenwick.len() {
                    fenwick[i] += 1; // adding one
                    i = get_next(i);
                }

                // yield the sum
                Some(sum)
            })
        .sum()
    }
}

fn main() {
    println!("{:?}", Solution::reverse_pairs(vec![1, 3, 2, 3, 1]));
    println!("{:?}", Solution::reverse_pairs(vec![2, 4, 3, 5, 1]));
    println!("{:?}", Solution::reverse_pairs(vec![-5, -5]));
}
