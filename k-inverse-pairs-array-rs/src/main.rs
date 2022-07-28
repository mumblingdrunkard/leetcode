// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/k-inverse-pairs-array/

struct Solution {}

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        const M: i64 = 1000000007;
        let array = (2..n + 1).fold(vec![1], |mut v, n| {
            let len = v.len();
            let mut current = vec![0; n];
            current.append(&mut v);
            current.append(&mut vec![0; n]);
            // will never need more than 1001 elements in the final result
            let res = (0..std::cmp::min(len + n - 1, 1001))
                .scan(0, |sum, i| {
                    *sum -= current[i];
                    *sum += current[i + n];
                    Some(*sum % M)
                })
                .collect();
            res
        });

        *array.get(k).unwrap_or(&0) as i32
    }
}

fn main() {
    println!("{}", Solution::k_inverse_pairs(1000, 2));
}
