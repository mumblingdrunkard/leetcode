// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/student-attendance-record-i/

struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent = 0;
        let mut late = 0;
        for c in s.chars() {
            match c {
                'A' => {
                    absent += 1;
                    late = 0;
                }
                'L' => late += 1,
                'P' => late = 0,
                _ => {}
            }

            if absent >= 2 {
                break;
            }

            if late >= 3 {
                break;
            }
        }

        late < 3 && absent < 2
    }
}

fn main() {
    println!("{}", Solution::check_record("PPALLP".to_string()));
}
