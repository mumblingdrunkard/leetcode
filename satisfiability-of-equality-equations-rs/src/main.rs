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
// https://leetcode.com/problems/satisfiability-of-equality-equations/

struct Solution {}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let (eq, ne): (Vec<_>, Vec<_>) = equations
            .into_iter()
            .partition(|s| &s.as_bytes()[1..3] == b"==");

        fn candidate(v: u8, m: &mut [u8; 256]) -> u8 {
            if m[v as usize] == v {
                return v;
            }
            let cand = candidate(m[v as usize], m);
            m[v as usize] = cand;
            cand
        }

        let mut p = [0; 256];
        p.iter_mut().enumerate().for_each(|(i, v)| *v = i as u8);

        // union find sets of equal variables
        for s in eq {
            let (v0, v1) = (s.as_bytes()[0], s.as_bytes()[3]);
            let cand0 = candidate(v0, &mut p);
            let cand1 = candidate(v1, &mut p);
            p[cand0 as usize] = cand1;
        }

        // verify that unequal variables don't lie within the same set
        for s in ne {
            let (v0, v1) = (s.as_bytes()[0], s.as_bytes()[3]);
            let cand0 = candidate(v0, &mut p);
            let cand1 = candidate(v1, &mut p);
            if cand0 == cand1 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let equations = ["b==a", "a==b"].into_iter().map(str::to_string).collect();
    println!("{}", Solution::equations_possible(equations));

    let equations = ["b==a", "a!=b"].into_iter().map(str::to_string).collect();
    println!("{}", Solution::equations_possible(equations));

    let equations = ["f==a", "a==b", "f!=e", "a==c", "b==e", "c==f"]
        .into_iter()
        .map(str::to_string)
        .collect();
    println!("{}", Solution::equations_possible(equations));
}
