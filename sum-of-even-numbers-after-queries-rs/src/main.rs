// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// This Source Code Form is “Incompatible With Secondary Licenses”, as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/sum-of-even-numbers-after-queries/

struct Solution {}

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(queries.len());
        let mut sum = nums.iter().filter(|&n| n % 2 == 0).sum::<i32>();

        for q in queries {
            let v = q[0];
            let i = q[1] as usize;

            if nums[i] % 2 == 0 {
                sum -= nums[i];
            }

            nums[i] += v;

            if nums[i] % 2 == 0 {
                sum += nums[i];
            }

            res.push(sum);
        }

        res
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
        )
    );
}
