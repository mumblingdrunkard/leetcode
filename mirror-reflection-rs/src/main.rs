// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/mirror-reflection/

struct Solution {}

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let g = Self::gcd(p, q);
        let (p, q) = (p / g, q / g);
        (p + 1) % 2 + q % 2
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        (b == 0).then(|| a).unwrap_or_else(|| Self::gcd(b, a % b))
    }

    #[allow(unused)]
    pub fn mirror_reflection_old(p: i32, q: i32) -> i32 {
        let mut side = 0;
        let mut height = 0;
        let mut direction = 1;
        loop {
            side ^= 1;
            height += direction * q;
            if height % p == 0 {
                break;
            }
            if height > p {
                height = p + p - height;
                direction *= -1;
            }
            if height < 0 {
                height *= -1;
                direction *= -1;
            }
        }
        if side == 0 && height == p {
            2
        } else if side == 1 && height == 0 {
            0
        } else {
            1
        }
    }
}

fn main() {
    println!("{}", Solution::mirror_reflection(1, 1));
    println!("{}", Solution::mirror_reflection(2, 1));
    println!("{}", Solution::mirror_reflection(2, 2));
    println!("{}", Solution::mirror_reflection(3, 1));
    println!("{}", Solution::mirror_reflection(3, 2));
    println!("{}", Solution::mirror_reflection(3, 3));
    println!("{}", Solution::mirror_reflection(4, 1));
    println!("{}", Solution::mirror_reflection(4, 2));
    println!("{}", Solution::mirror_reflection(4, 3));
    println!("{}", Solution::mirror_reflection(4, 4));
}
