// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/bag-of-tokens/

struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort();

        let mut score = 0;

        // two pointers, we turn i's face up to gain score, turn j's up to gain power
        let (mut i, mut j) = (0, tokens.len());
        while i < j {
            // try to gain power first
            if tokens[i] <= power {
                power -= tokens[i];
                score += 1;
                i += 1;
            } else if score > 0 && j - i > 1 {
                // not worth it to lose one score if we can't gain
                // it back in the next turn
                j -= 1;
                power += tokens[j];
                score -= 1;
            } else {
                // we cannot gain any more points
                break;
            }
        }

        score
    }
}

fn main() {
    println!("{}", Solution::bag_of_tokens_score(vec![100], 50));

    println!("{}", Solution::bag_of_tokens_score(vec![100, 200], 150));

    println!(
        "{}",
        Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200)
    );
}
