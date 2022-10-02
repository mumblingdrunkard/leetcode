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
// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/

struct Solution {}

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const M: usize = 1000000007;
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;

        let mut q = &mut vec![0; target + k + 2];
        let mut p = &mut vec![0; target + k + 2];

        (1..=k).for_each(|i| {
            q[i] = 1;
        });

        for _ in 1..n {
            for i in 1..=target {
                for j in 1..=k {
                    p[i + j] += q[i];
                    p[i + j] %= M;
                }
            }

            std::mem::swap(&mut p, &mut q);
            p.fill(0);
        }

        (q[target] % M) as i32
    }
}

fn main() {
    println!("{}", Solution::num_rolls_to_target(1, 6, 3));
    println!("{}", Solution::num_rolls_to_target(2, 6, 7));
    println!("{}", Solution::num_rolls_to_target(30, 30, 500));
}
