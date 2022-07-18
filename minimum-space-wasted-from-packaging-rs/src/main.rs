// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/minimum-space-wasted-from-packaging/

struct Solution {}

impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1000000007;

        // allow mutation
        let (mut packages, mut boxes) = (packages, boxes);

        // sort the packages
        packages.sort();

        // calculate the cumulative sum, prepend 0 so that cumulative_size[n] is the total size of
        // the first n packages
        let cums: Vec<_> = [0]
            .iter()
            .chain(packages.iter())
            .scan(0i64, |total, &size| {
                *total = *total + size as i64;
                Some(*total)
            })
            .collect();

        // sort all suppliers
        boxes.iter_mut().for_each(|s| s.sort());

        (boxes
            .iter() // for all suppliers
            // filter out the incompatible suppliers
            .filter(|supplier| supplier.last().unwrap() >= packages.last().unwrap())
            // map each supplier to their total wasted space, by
            .map(|supplier| {
                supplier
                    .iter() // for each box size in the supplier
                    .scan(0, |split, &box_size| {
                        // find number of packages smaller or equal to box_size
                        let new_split = packages.partition_point(|&p| p <= box_size);
                        let n = new_split - *split; // number of packages in this split
                        // calculate total space used by boxes for packages
                        // NB: use i64 internally as .min() will get wrong value if we % the total
                        let total = n as i64 * box_size as i64;
                        // wasted space is equal to the size of all the boxes minus the cumulative
                        // size of the packages between the previous and the current split
                        let wasted = total - (cums[new_split] - cums[*split]);
                        *split = new_split; // update the split
                        Some(wasted) // yield the wasted space
                    })
                    .fold(0, |sum, wasted| sum + wasted) // accumulate the wasted space
            })
            .min() // take the minimum of all suppliers
            .unwrap_or(-1) // or return -1 if no compatible suppliers were found
            % M) as i32 // mod 10⁹+7 per problem requirements and cast to i32
    }
}

fn main() {
    let mut packages = vec![0; 100000];
    packages
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = i as i32 + 1);

    let mut suppliers = vec![vec![0; 2]; 50000];
    suppliers
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = vec![i as i32 + 50000, 100000]);

    println!("{}", Solution::min_wasted_space(packages, suppliers));
}
