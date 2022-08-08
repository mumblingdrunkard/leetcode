// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/unique-paths/

struct Solution {}

impl Solution {
    // calculates (n+m-2)!/((n-1)!(m-1)!)
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (n, m) = (n as i64, m as i64);

        let mut divisors = 2..m;
        let mut div = divisors.next();

        // multipliers n and above, this lets us ignore the (n-1)! part for the divisor
        (n..m + n - 1).fold(1i64, |mut result, m| {
            // multiply by the next multiplier
            result *= m;

            // divide the result if possible
            if let Some(d) = div {
                if result % d == 0 {
                    result /= d;
                    div = divisors.next();
                }
            }

            result
        }) as i32
    }
}

fn main() {
    println!("{}", Solution::unique_paths(7, 7));
    println!("{}", Solution::unique_paths(3, 7));
    println!("{}", Solution::unique_paths(2, 2));
}
