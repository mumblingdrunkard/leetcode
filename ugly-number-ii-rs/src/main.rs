// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/ugly-number-ii/

struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut p = [0, 0, 0];
        let mul = [2, 3, 5];
        let mut num = vec![0; n];
        num[0] = 1;
        for i in 1..n {
            let next = p
                .iter()
                .enumerate()
                .map(|(j, &k)| num[k] * mul[j])
                .min()
                .unwrap();
            num[i] = next;
            p.iter_mut().enumerate().for_each(|(j, k)| {
                if num[*k] * mul[j] == num[i] {
                    *k += 1;
                }
            });
        }

        num[n - 1]
    }

    #[allow(unused)]
    pub fn nth_ugly_number_bad(n: i32) -> i32 {
        let n = n as usize;
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashSet};

        let mut seen = HashSet::new();
        let mut numbers = BinaryHeap::new();
        numbers.push(Reverse(1u64));
        seen.insert(1u64);
        for _ in 0..1500 {
            if let Some(Reverse(n)) = numbers.pop() {
                for m in [2, 3, 5] {
                    if !seen.contains(&(n * m)) {
                        numbers.push(Reverse(n * m));
                        seen.insert(n * m);
                        continue;
                    }
                }
            }
        }

        let mut numbers = seen.into_iter().collect::<Vec<_>>();
        numbers.sort();
        numbers[n - 1] as i32
    }
}

fn main() {
    println!("{}", Solution::nth_ugly_number(10));
}
