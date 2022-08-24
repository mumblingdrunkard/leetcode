// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/power-of-three/

struct Solution {}

impl Solution {
    // this approach seems to be faster, likely because the % operation is costly
    // usually marked as faster than 95% or more
    pub fn is_power_of_three(n: i32) -> bool {
        // all powers of 3 less than i32::MAX
        const POWERS: [i32; 20] = {
            let mut powers = [0; 20];
            powers[0] = 1;
            let mut i = 1;
            while i < 20 {
                powers[i] = powers[i - 1] * 3;
                i += 1;
            }
            powers
        };

        // basically a hashset, but the hashing operation is `&` and it is very cheap
        // 128 is the lowest power of 2 that maps all 3^n < i32::MAX uniquely
        const POWERS2: [i32; 128] = {
            let mut powers = [0; 128];
            let mut i = 0;
            while i < 20 {
                powers[POWERS[i] as usize & 0x7F] = POWERS[i];
                i += 1;
            }
            powers
        };

        n > 0 && POWERS2[n as usize & 0x7F] == n
    }

    #[allow(unused)]
    // solution with "magic" number
    pub fn is_power_of_three_w_magic_number(n: i32) -> bool {
        // largest power of three less than i32::MAX
        const MAGIC_NUMBER: i32 = {
            let mut number = 1;
            while number < i32::MAX / 3 {
                number *= 3;
            }
            number
        };

        // MAGIC_NUMBER == 3^n, (only composed of 3s),
        // thus it can only be divided by 3^k where k <= n.
        n > 0 && MAGIC_NUMBER % n == 0
    }
}

fn main() {
    println!("{}", Solution::is_power_of_three(27));
}
