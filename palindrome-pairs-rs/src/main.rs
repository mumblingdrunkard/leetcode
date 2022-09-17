// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/palindrome-pairs/

struct Solution {}

use std::collections::HashMap;

struct Trie {
    elements: Vec<u16>,
    children: HashMap<u8, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            elements: vec![],
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, s: &[u8], id: u16) {
        if !s.is_empty() {
            self.children
                .entry(s[0])
                .or_insert_with(Trie::new)
                .insert(&s[1..], id);
        } else {
            self.elements.push(id);
        }
    }

    fn family(&self, s: &[u8]) -> Vec<u16> {
        let mut res = self.elements.clone();
        if s.is_empty() {
            res.extend(self.children.values().flat_map(|v| v.family(s)));
        } else if let Some(child) = self.children.get(&s[0]) {
            res.append(&mut child.family(&s[1..]));
        }
        res
    }
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let words = words
            .into_iter()
            .map(String::into_bytes)
            .collect::<Vec<_>>();

        let mut t = Trie::new();

        for (i, w) in words.iter().enumerate() {
            t.insert(w, i as u16);
        }

        let reversed = words
            .iter()
            .map(|w| w.iter().rev().copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut result = vec![];

        for (j, w) in reversed.iter().enumerate() {
            let supers = t.family(w);
            for i in supers {
                let i = i as usize;
                if i == j {
                    continue;
                }
                let a = &words[i];
                let b = &reversed[j];
                // the length of the common prefix
                let common = std::cmp::min(a.len(), b.len());
                if Self::is_palindrome(&a[common..]) && Self::is_palindrome(&b[common..]) {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }

        result
    }

    fn is_palindrome(s: &[u8]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let words = ["abcd", "dcba", "lls", "s", "sssll"]
        .into_iter()
        .map(str::to_string)
        .collect();

    println!("{:?}", Solution::palindrome_pairs(words));

    let words = ["bat", "tab", "cat"]
        .into_iter()
        .map(str::to_string)
        .collect();

    println!("{:?}", Solution::palindrome_pairs(words));

    let words = ["a", ""].into_iter().map(str::to_string).collect();

    println!("{:?}", Solution::palindrome_pairs(words));

    let words = ["s", "ss", "sss", "ssss", "sssss", "ssssss", "sssssss"]
        .into_iter()
        .map(str::to_string)
        .collect();

    println!("{:?}", Solution::palindrome_pairs(words));
}
