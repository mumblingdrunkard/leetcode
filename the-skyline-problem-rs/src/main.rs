// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// This Source Code Form is “Incompatible With Secondary Licenses”, as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/the-skyline-problem/

struct Solution {}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::btree_map::Entry;
        use std::collections::BTreeMap;

        #[derive(Copy, Clone, Debug)]
        enum Edge {
            R,
            L,
        }

        let mut edges = buildings
            .into_iter()
            .flat_map(|v| [(v[0], v[2], Edge::L), (v[1], v[2], Edge::R)])
            .collect::<Vec<_>>();

        edges.sort_by_key(|t| t.0);

        let mut edges = edges.into_iter();
        let mut curr = edges.next();

        let mut open = BTreeMap::new();

        let mut res = vec![];

        let mut prev = 0;

        while let Some((x, _y, _e)) = curr {
            let curr_x = x;

            while let Some((_x, y, e)) = curr {
                match e {
                    Edge::L => *open.entry(y).or_insert(0) += 1,
                    Edge::R => match open.entry(y) {
                        Entry::Occupied(mut ent) if *ent.get() > 1 => {
                            *ent.get_mut() -= 1;
                        }
                        _ => {
                            open.remove(&y);
                        }
                    },
                }

                curr = edges.next();

                if let Some((x, _y, _e)) = curr {
                    if x != curr_x {
                        break;
                    }
                } else {
                    break;
                }
            }

            if let Some((&k, _)) = open.iter().next_back() {
                if prev != k {
                    res.push(vec![x, k]);
                    prev = k;
                }
            } else {
                res.push(vec![x, 0]);
                prev = 0;
            }
        }

        res
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8]
        ])
    );

    println!(
        "{:?}",
        Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3],])
    );
}
