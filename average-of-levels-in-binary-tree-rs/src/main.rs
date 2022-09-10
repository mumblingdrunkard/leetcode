// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/average-of-levels-in-binary-tree/

struct Solution {}

pub struct TreeNode {
    pub val: i32,
    pub left: N,
    pub right: N,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn wrapped(val: i32) -> N {
        Some(Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        })))
    }

    pub fn with(val: i32, left: N, right: N) -> N {
        Some(Rc::new(RefCell::new(Self { val, left, right })))
    }
}

use std::{cell::RefCell, collections::VecDeque, rc::Rc};
type N = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn average_of_levels(root: N) -> Vec<f64> {
        let mut v = Vec::new();
        let mut q = VecDeque::new();
        q.push_back((root, 0));

        while let Some((parent, level)) = q.pop_front() {
            if let Some(node) = parent {
                v.push((node.borrow().val, level));
                q.push_back((node.borrow().left.clone(), level + 1));
                q.push_back((node.borrow().right.clone(), level + 1));
            }
        }

        let mut levels = vec![vec![]; v.last().unwrap().1 + 1];
        for (val, level) in v {
            levels[level].push(val);
        }

        levels
            .into_iter()
            .map(|l| {
                let n = l.len();
                l.into_iter().map(|i| i as f64).sum::<f64>() / n as f64
            })
            .collect()
    }
}

fn main() {
    let tree = TreeNode::with(
        3,
        TreeNode::with(9, TreeNode::wrapped(15), TreeNode::wrapped(7)),
        TreeNode::wrapped(20),
    );
    println!("{:?}", Solution::average_of_levels(tree));
}
