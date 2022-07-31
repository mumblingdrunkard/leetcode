// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at
// https://leetcode.com/problems/maximum-product-after-k-increments/

struct Solution {}

impl Solution {
    // the idea here is to order the numbers from smallest to largest and "pour" k units of water
    // into the container.
    //
    // E.g. nums: [2, 1, 6, 3], k: 6
    //
    // sort the numbers:
    // h = [1, 2, 3, 6]
    //
    // model bar-plot:
    //
    //    *
    //    *
    //    *
    //   **
    //  ***
    // ****
    //
    // let `h[i]` be the height of bar `i`
    // let `v[i]` be the volume *up to*, but not including `h[i]`
    //
    //     v = [0, 1, 3, 6, 12]
    //
    // let the missing volume `mv` for a bar `i` be such that `mv[i] + v[i] = h[i] × i`;
    //
    //     mv = [0, 1, 3, 12]
    //
    // now, find the point where the missing volume is larger than `k`
    //
    //     let mut i = 0;
    //     while i < h.len() && mv[i] < k {
    //         i += 1;
    //     }
    //
    // after this, `i == 3` because 3 × 6 - 6 is 12, which is larger than 6
    // the total volume *after* filling the first i bars will be equal to `v[i] + k`
    // thus, we can calculate the height of the filled area as `(v[i] + k) / i = 12 / 3 = 4`
    // now, simply fill each bar up to the set height:
    //
    //    *
    //    *
    // ****
    // ****
    // ****
    // ****
    //
    // what if `k = 8`?
    //
    // in this case we will have a remainder after the division `(v[i] + k) / i` which we can
    // capture as:
    //
    //     rem = (v[i] + k) % i; // = 2
    //
    // then we can fill in the missing increments by adding 1 to the first `rem` values
    //
    //    *
    // ** *
    // ****
    // ****
    // ****
    // ****
    //
    // finally calculate the product
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = nums;
        let k = k as usize;

        // sort the numbers so we can find where the partition point is
        h.sort();

        // find the point where k runs out
        let (fill, up_to, remainder) = {
            let mut volume = 0; // volume without k
            let mut i = 0;
            while i < h.len() && i as i64 * h[i] as i64 - volume < k as i64 {
                volume += h[i] as i64;
                i += 1;
            }
            (i, (volume as usize + k) / i, (volume as usize + k) % i)
        };

        // fill the container
        for i in 0..fill {
            h[i] = up_to as i32;
        }

        // top up with the remainder
        for i in 0..remainder {
            h[i] += 1;
        }

        // calculate the total product mod 10⁹+7
        h.into_iter()
            .fold(1i64, |result, n| (result * n as i64) % 1000000007) as i32
    }
}

fn main() {
    println!("{}", Solution::maximum_product(vec![6, 3, 3, 2], 2));
    println!("{}", Solution::maximum_product(vec![1, 2, 3, 4, 5, 20], 40));
    println!("{}", Solution::maximum_product(vec![1000000; 100000], 100000));
}
