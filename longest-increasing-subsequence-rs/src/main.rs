// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/longest-increasing-subsequence/

struct Solution {}

// Fenwick/BI tree to efficiently track and query maxima in the range [0..i]
struct Fenwick {
    fen: Vec<i32>,
}

impl Fenwick {
    fn set(&mut self, i: usize, n: i32) {
        let mut i = i + 1;
        while i < self.fen.len() {
            self.fen[i] = std::cmp::max(self.fen[i], n);
            i += i & i.overflowing_neg().0; // next element
        }
    }

    fn max(&self, i: usize) -> i32 {
        let mut i = i + 1;
        let mut max = i32::MIN;
        while i > 0 {
            max = std::cmp::max(max, self.fen[i]); // accumulate the sum
            i -= i & i.overflowing_neg().0; // parent element
        }
        max
    }

    fn with_size(sz: usize) -> Self {
        Self {
            fen: vec![0; sz + 2],
        }
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // simple way to map all numbers to a contiguous range
        // do this because 2500 << 2 × 10⁴
        let mut map = nums.clone();
        map.sort();
        map.dedup();

        nums.iter()
            .scan(Fenwick::with_size(map.len()), |fen, n| {
                let index = map.binary_search(n).unwrap() + 1;
                let maxlt = fen.max(index - 1);
                fen.set(index, maxlt + 1);
                Some(maxlt + 1)
            })
            .max()
            .unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );

    println!("{}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));

    println!("{}", Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
}
