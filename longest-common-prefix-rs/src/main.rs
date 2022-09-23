// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/longest-common-prefix/

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|cum, s| {
                cum.bytes()
                    .zip(s.bytes())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a as char)
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn main() {
    let strs = ["flowers", "flow", "flowery"]
        .into_iter()
        .map(str::to_string)
        .collect();
    println!("{}", Solution::longest_common_prefix(strs));
}
