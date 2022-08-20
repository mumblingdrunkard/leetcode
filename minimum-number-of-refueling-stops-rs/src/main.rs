// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/minimum-number-of-refueling-stops/

struct Solution {}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        // track "visited" stations, ordered by how much fuel they have
        let mut q = BinaryHeap::new();

        // how far we can drive in the current iteration
        let mut reach = start_fuel;

        // next unvisited station
        let mut i = 0;

        // duh...
        let mut stops = 0;

        // while we still can't reach the target
        while reach < target {
            // while there are stations ahead of us that we can reach
            while i < stations.len() && stations[i][0] <= reach {
                // track their fuel
                q.push(stations[i][1]);
                // check the next station
                i += 1;
            }

            // if we cannot reach any more stations, we're done
            if q.is_empty() {
                return -1;
            }

            // take fuel from the station that has the most fuel of those we have visited
            reach += q.pop().unwrap();
            stops += 1;
        }

        stops
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_refuel_stops(
            100,
            10,
            vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
        )
    );

    println!(
        "{}",
        Solution::min_refuel_stops(100, 50, vec![vec![25, 30]])
    );

    println!(
        "{}",
        Solution::min_refuel_stops(
            1000,
            299,
            vec![
                vec![13, 21],
                vec![26, 115],
                vec![100, 47],
                vec![225, 99],
                vec![299, 141],
                vec![444, 198],
                vec![608, 190],
                vec![636, 157],
                vec![647, 255],
                vec![841, 123]
            ]
        )
    );
}
