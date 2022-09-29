// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/find-k-closest-elements/

struct Solution {}

impl Solution {
    pub fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        arr.sort();
        let i = match arr.binary_search(&x) {
            Ok(v) => v,
            Err(v) => v,
        };

        let mut start = i;
        let mut end = i;

        while end - start < k {
            if start == 0 {
                end += 1;
                continue;
            }

            if end == arr.len() {
                start -= 1;
                continue;
            }

            if (arr[end] - x).abs() < (arr[start - 1] - x).abs() {
                end += 1;
            } else {
                start -= 1;
            }
        }

        arr[start..end].to_vec()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 4)
    );
}
