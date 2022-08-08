// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/network-delay-time/

struct Solution {}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut adj = vec![vec![]; n as usize];
        for t in times.iter() {
            let (u, v, w) = (t[0] as usize, t[1] as usize, t[2]);
            adj[u - 1].push((v - 1, w));
        }

        let mut times = vec![-1; n as usize];

        let mut q = BinaryHeap::new();
        q.push(Reverse((k as usize - 1, 0)));

        while let Some(Reverse((u, t))) = q.pop() {
            if times[u] >= 0 && t >= times[u] {
                continue;
            }

            times[u] = t;
            for (v, w) in adj[u].iter() {
                q.push(Reverse((*v, t + w)));
            }
        }

        let impossible = times.iter().any(|&x| x == -1);
        if impossible {
            -1
        } else {
            *times.iter().max().unwrap()
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2)
    );
}
