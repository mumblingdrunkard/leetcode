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
// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/

struct Solution {}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        // ignore leading and trailing plants
        let corridor = corridor.trim_matches('P');

        let corridor = corridor.chars().collect::<Vec<_>>();

        if corridor.len() < 2 {
            // there cannot possibly be 2 seats
            return 0;
        }

        // runs contains the number of cuts between each pair of seats
        let mut runs = vec![];
        let mut i = 0;
        while i < corridor.len() {
            // only make cuts between pairs of seats
            let mut seats = 0;
            while i < corridor.len() && seats != 2 {
                if corridor[i] == 'S' {
                    seats += 1;
                }
                i += 1;
            }

            if seats < 2 {
                // there aren't enough seats
                return 0;
            }

            let mut plants = 0;
            while i < corridor.len() && corridor[i] != 'S' {
                plants += 1;
                i += 1;
            }

            // with n plants between two pairs of seats, we can make n + 1 cuts
            runs.push(plants + 1);
        }

        // simple combinatorial calculation
        runs.into_iter().fold(1i64, |res, n| (res * n) % 1000000007) as i32
    }
}

fn main() {
    println!("{}", Solution::number_of_ways("SSPPSPS".to_string()));
    println!("{}", Solution::number_of_ways("PPSPSP".to_string()));
    println!("{}", Solution::number_of_ways("P".to_string()));
}
