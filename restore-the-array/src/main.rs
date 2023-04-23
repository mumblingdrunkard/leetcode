// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/two-sum/

struct Solution;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        const MOD: usize = 1000000007;
        let k = k as usize;
        let chars = s.chars().collect::<Vec<_>>();

        let mut invalid = false;

        let nums = chars
            .iter()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .scan(1usize, |mul, v| {
                if v == 0 {
                    *mul *= 10;
                    invalid |= *mul > k;
                    Some(None)
                } else {
                    let m = *mul;
                    *mul = 1;
                    Some(Some((m * v as usize, m)))
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        if nums[0].0 > k || invalid {
            return 0;
        }

        // stores the current lineages
        let mut lineages = vec![(nums[0].0, nums[0].1, 1usize)];

        for (value, order) in nums.iter().copied().skip(1) {
            // continue valid lineages
            let mut new = vec![];
            if value <= k {
                let sum = lineages.iter().map(|l| l.2).sum::<usize>();
                new.push((value, order, sum % MOD));
            }

            for (tail, tail_order, tail_lineage) in lineages {
                let continuation = value * tail_order * 10 + tail;
                if continuation <= k {
                    new.push((continuation, order * tail_order * 10, tail_lineage));
                }
            }

            lineages = new;
        }

        let mut sum = 0;
        for (_, _, tail_lineages) in lineages {
            sum += tail_lineages;
            sum %= MOD;
        }

        sum as i32
    }
}

fn main() {
    println!("{}", Solution::number_of_arrays("1317".to_string(), 200));
    println!(
        "{}",
        Solution::number_of_arrays("10000000000000000000".to_string(), 1000000000)
    );
    println!("{}", Solution::number_of_arrays("1000".to_string(), 10));
}
