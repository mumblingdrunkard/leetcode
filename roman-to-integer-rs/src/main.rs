// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/roman-to-integer/

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert('M', 1000);
        map.insert('D', 500);
        map.insert('C', 100);
        map.insert('L', 50);
        map.insert('X', 10);
        map.insert('V', 5);
        map.insert('I', 1);

        let mut result = 0;
        let mut iter = s.chars();
        let mut next = iter.next();
        while let Some(c) = next {
            next = iter.next();
            if let Some(d) = next {
                if map.get(&d).unwrap() > map.get(&c).unwrap() {
                    result += map.get(&d).unwrap() - map.get(&c).unwrap();
                    next = iter.next();
                    continue;
                }
            }
            result += map.get(&c).unwrap();
        }

        result
    }
}

fn main() {
    println!("{}", Solution::roman_to_int("MCMXCIV".to_string()));
}
