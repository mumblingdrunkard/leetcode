// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/find-duplicate-file-in-system/

struct Solution {}

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for path in &paths {
            let mut parts = path.split(' ');
            let root = parts.next().unwrap();
            for part in parts {
                let mut parts = part.split('(');
                let filename = parts.next().unwrap();
                let contents = parts.next().unwrap().trim_end_matches(')').to_string();
                m.entry(contents)
                    .or_insert(vec![])
                    .push(format!("{}/{}", root, filename));
            }
        }

        m.values().filter(|v| v.len() > 1).cloned().collect()
    }
}

fn main() {
    let paths = [
        "root/a 1.txt(abcd) 2.txt(efgh)",
        "root/c 3.txt(abcd)",
        "root/c/d 4.txt(efgh)",
        "root 4.txt(efgh)",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    println!("{:?}", Solution::find_duplicate(paths));
}
