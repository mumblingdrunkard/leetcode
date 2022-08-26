// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/reordered-power-of-2/

struct Solution {}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        // not strictly necessary, but it's nice to have
        if n < 1 {
            return false;
        }

        let mut n = n as usize;

        // POWER_DIGITS is an array of 30 numbers where each number, when represented in octal is
        // itself an array of counts of digits in each power of 2
        // The left-most digit will be the count of 9s
        // This array is computed at compile time
        const POWER_DIGITS: [u32; 30] = {
            let mut powers = [[0; 10]; 30];
            let mut i = 0;
            while i < powers.len() {
                let mut p = 1 << i;
                while p > 0 {
                    powers[i][p % 10] += 1;
                    p /= 10;
                }
                i += 1;
            }

            let mut power_digits = [0u32; 30];
            let mut i = 0;
            while i < powers.len() {
                let mut n = 0;
                let mut j = 0;
                while j < 10 {
                    n = n << 3;
                    n |= powers[i][j] as u32;
                    j += 1;
                }
                power_digits[i] = n;
                i += 1;
            }

            // sort the array with bubble sort
            // (can't use .sort() inside const)
            loop {
                let mut i = 1;
                let mut swapped = false;
                while i < powers.len() {
                    if power_digits[i - 1] > power_digits[i] {
                        let a = power_digits[i - 1];
                        let b = power_digits[i];
                        power_digits[i - 1] = b;
                        power_digits[i] = a;
                        swapped = true;
                    }
                    i += 1;
                }

                if !swapped {
                    break;
                }
            }

            power_digits
        };

        // uncomment to print the contents of POWER_DIGITS with an octal representation
        // for p in POWER_DIGITS {
        //     println!("{p:10o}");
        // }

        // get digit counts
        let mut power_digits = [0; 10];
        while n > 0 {
            power_digits[n % 10] += 1;
            n /= 10;
        }

        // no powers of 2 <= 2^30 have more than 4 of any digit
        if power_digits.iter().any(|&c| c > 4) {
            return false;
        }

        // transform to octal where each digit represents the count of some digit in the original
        // number
        let mut n = 0;
        for b in power_digits {
            n = n << 3;
            n |= b as u32;
        }

        // if n is a reordered power of 2, POWER_DIGITS should contain it
        POWER_DIGITS.binary_search(&n).is_ok()
    }
}

fn main() {
    println!("{}", Solution::reordered_power_of2(16));
    println!("{}", Solution::reordered_power_of2(71));
}
