// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/maximum-performance-of-a-team/

struct Solution {}

impl Solution {
    pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        // approach:
        // sort engineers on efficiency
        // for each engineer in the reversed list, add it to the group, then if the group is
        // overfull, remove the engineer with the lowest speed (maximise the sum of the group) then
        // multiply by the efficiency of this engineer (guaranteed to be the lowest efficiency in
        // the group because we sorted).

        const M: i64 = 1000000007;
        let mut engineers = efficiency
            .into_iter()
            .zip(speed.into_iter())
            .collect::<Vec<_>>();

        engineers.sort();

        (engineers
            .into_iter()
            .rev()
            .scan(
                (std::collections::BinaryHeap::new(), 0),
                |(q, sum), (e, s)| {
                    q.push(std::cmp::Reverse(s as i64));

                    *sum += s as i64;

                    if q.len() > k as usize {
                        *sum -= q.pop().unwrap().0;
                    }

                    // e will always be the minimum efficiency
                    Some(*sum * e as i64)
                },
            )
            .max()
            .unwrap()
            % M) as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2)
    );
}
