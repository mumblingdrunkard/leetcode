// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/build-array-from-permutation/

struct Solution {}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums;
        let z = n.len() as i32;

        // set each index such that n[i] % z = n[i]
        // and n[i] / z = n[n[i]]
        for i in 0..n.len() {
            // extract the value of n[n[i]]
            let k = n[n[i] as usize] % z as i32;

            n[i] = z * k + n[i];
        }

        // extract n[n[i]] for each n[i]
        for i in 0..n.len() {
            n[i] /= z;
        }

        n
    }
}

fn main() {
    println!("{:?}", Solution::build_array(vec![0, 2, 1, 5, 3, 4]));
}
