// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/string-to-integer-atoi/

struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        let mut chars = s.chars();
        let mut result = 0;

        let sign = match chars.next() {
            Some('-') => -1,
            Some('+') => 1,
            Some(c) => {
                if let Some(d) = c.to_digit(10) {
                    result += d as i32;
                } else {
                    return 0;
                }
                1
            }
            None => return 0,
        };

        while let Some(c) = chars.next() {
            if let Some(d) = c.to_digit(10) {
                if sign == 1 {
                    if i32::MAX / 10 < result {
                        result = i32::MAX;
                        break;
                    }

                    result *= 10;

                    if (i32::MAX - d as i32) < result {
                        result = i32::MAX;
                        break;
                    }

                    result += d as i32;
                } else if sign == -1 {
                    if i32::MIN / 10 > result {
                        result = i32::MIN;
                        break;
                    }

                    result *= 10;

                    if (i32::MIN + d as i32) > result {
                        result = i32::MIN;
                        break;
                    }

                    result -= d as i32;
                }
            } else {
                break;
            }
        }

        result
    }
}

fn main() {
    println!("{}", Solution::my_atoi(".1".to_string()));
}
