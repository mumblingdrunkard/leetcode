// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/reverse-integer/

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn reverse_neg(mut x: i32) -> i32 {
            let mut result = 0;

            while x != 0 {
                let rem = x % 10;
                if i32::MIN / 10 > result {
                    break;
                }
                result *= 10;

                if i32::MIN - rem > result {
                    break;
                }

                result += rem;
                x /= 10;
            }

            if x == 0 {
                result
            } else {
                0
            }
        }

        fn reverse_pos(mut x: i32) -> i32 {
            let mut result = 0;
            while x != 0 {
                let rem = x % 10;
                if i32::MAX / 10 < result {
                    break;
                }
                result *= 10;

                if i32::MAX - rem < result {
                    break;
                }

                result += rem;
                x /= 10;
            }

            if x == 0 {
                result
            } else {
                0
            }
        }

        if x < 0 {
            reverse_neg(x)
        } else {
            reverse_pos(x)
        }
    }
}

fn main() {
    println!("{}", Solution::reverse(-120));
}
