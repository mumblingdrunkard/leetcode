// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/binary-trees-with-factors/

struct Solution {}

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        const M: usize = 1000000007;
        let mut trees = std::collections::HashMap::new();
        arr.sort();
        let arr: Vec<_> = arr.iter().map(|&n| n as usize).collect();
        for &n in &arr {
            let mut sum = 1;
            for &p in &arr {
                if p * 2 > n {
                    break;
                }

                if n % p != 0 {
                    continue;
                }

                let q = n / p;

                let c1 = trees.get(&p).unwrap_or(&0);
                let c2 = trees.get(&q).unwrap_or(&0);
                sum += c1 * c2;
                sum %= M;
            }
            trees.insert(n, sum);
        }

        (trees.iter().map(|(_, v)| v).sum::<usize>() % M) as i32
    }
}

fn main() {
    println!("{}", Solution::num_factored_binary_trees(vec![2, 4, 5, 10]));
}

// 2, 4, 16?
//
// [2] [4] [16]
// [4, 2, 2]
// [16, 4, 4]
//
// [16, 4, 4, 2, 2]
// [16, 4, 4, /, /, 2, 2]
// [16, 4, 4, 2, 2, 2, 2]
//
//
// [2, 3, 6, 36]
//
// [2] [3] [6] [36]
//
// [6, 2, 3], [6, 3, 2]
//
// [36, 6, 6]
// [36, 6, 6, 2, 3]
// [36, 6, 6, 3, 2]
// [36, 6, 6, /, /, 2, 3]
// [36, 6, 6, /, /, 3, 2]
// [36, 6, 6, 2, 3, 2, 3]
// [36, 6, 6, 2, 3, 3, 2]
// [36, 6, 6, 3, 2, 2, 3]
// [36, 6, 6, 6, 2, 3, 2]
