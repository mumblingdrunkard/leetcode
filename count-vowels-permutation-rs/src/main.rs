// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/count-vowels-permutation/

struct Solution {}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        Self::count_vowel_permutation_log(n as usize) as i32
    }

    #[allow(unused)]
    pub fn count_vowel_permutation_linear(n: i32) -> i32 {
        // 0 -> 'a'
        // 1 -> 'e'
        // 2 -> 'i'
        // 3 -> 'o'
        // 4 -> 'u'
        // =>
        // 0 can follow { 1, 2, 4 }
        // 1 can follow { 0, 2 }
        // 2 can follow { 1, 3 }
        // 3 can follow { 2 }
        // 4 can follow { 2, 3 }
        //
        const M: i32 = 1000000007;
        let n = n as usize;

        let p = (1..n).fold((1, 1, 1, 1, 1), |p, _| {
            (
                ((p.1 + p.2) % M + p.4) % M,
                (p.0 + p.2) % M,
                (p.1 + p.3) % M,
                p.2,
                (p.2 + p.3) % M,
            )
        });

        // this is ducking fugly
        (((p.0 + p.1) % M + (p.2 + p.3) % M) % M + p.4) % M
    }

    #[allow(unused)]
    fn log2(n: usize) -> usize {
        let mut log = 1;
        let mut k = 1;
        while k < n {
            log += 1;
            k *= 2;
        }
        log
    }

    #[allow(unused)]
    pub fn count_vowel_permutation_log(n: usize) -> usize {
        const M: usize = 1000000007;
        let follows = vec![
            vec![1usize, 2, 4],
            vec![0, 2],
            vec![1, 3],
            vec![2],
            vec![2, 3],
        ];

        let mut p: Vec<[[usize; 5]; 5]> = vec![[[0; 5]; 5]];

        p[0][0][0] = 1;
        p[0][1][1] = 1;
        p[0][2][2] = 1;
        p[0][3][3] = 1;
        p[0][4][4] = 1;

        for l in 1..Self::log2(n) {
            // 0 can be followed by { 1 }
            // 1 can be followed by { 0, 2 }
            // 2 can be followed by { 0, 3, 4 }
            // 3 can be followed by { 2, 4 }
            // 4 can be followed by { 0 }
            let mut next = [[0; 5]; 5];
            for i in 0..5 {
                for j in 0..5 {
                    let mut sum = 0;
                    for a in 0..5 {
                        for &b in &follows[a] {
                            sum += (p[l - 1][i][a] * p[l - 1][b][j]) % M;
                            sum %= M;
                        }
                    }
                    next[i][j] = sum;
                }
            }
            p.push(next);
        }

        let mut iter = p
            .into_iter()
            .enumerate()
            .filter_map(|(i, r)| ((n >> i) & 1 == 1).then(|| r));

        let first = iter.next().unwrap();
        let res = iter.fold(first, |c, r| {
            let mut next = [[0; 5]; 5];
            for i in 0..5 {
                for j in 0..5 {
                    let mut sum = 0;
                    for a in 0..5 {
                        for &b in &follows[a] {
                            sum += (c[i][a] * r[b][j]) % M;
                            sum %= M;
                        }
                    }
                    next[i][j] = sum;
                }
            }
            next
        });

        res.into_iter()
            .map(|r| r.into_iter().sum::<usize>() % M)
            .sum::<usize>()
            % M
    }
}

fn main() {
    println!("{}", Solution::count_vowel_permutation(5));
}
