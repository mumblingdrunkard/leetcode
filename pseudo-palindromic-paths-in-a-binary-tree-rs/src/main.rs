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
// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/

struct Solution {}

#[derive(Debug, PartialEq, Eq)]
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

use std::{cell::RefCell, rc::Rc};
type N = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn pseudo_palindromic_paths(root: N) -> i32 {
        Self::recursive(root, &mut [false; 9], 0)
    }

    pub fn recursive(root: N, state: &mut [bool; 9], mut count: usize) -> i32 {
        if let Some(node) = root {
            let i = node.borrow().val as usize - 1;
            state[i] = !state[i];
            if state[i] {
                count += 1;
            } else {
                count -= 1;
            }

            let res = if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if count > 1 {
                    0
                } else {
                    1
                }
            } else {
                Self::recursive(node.borrow().left.clone(), state, count)
                    + Self::recursive(node.borrow().right.clone(), state, count)
            };

            state[i] = !state[i]; // revert myself

            res
        } else {
            0
        }
    }
}

fn main() {
    let tree = TreeNode::with(
        2,
        TreeNode::with(3, TreeNode::wrapped(3), TreeNode::wrapped(1)),
        TreeNode::with(1, None, TreeNode::wrapped(1)),
    );

    println!("{}", Solution::pseudo_palindromic_paths(tree));

    let tree = TreeNode::with(
        2,
        TreeNode::with(
            1,
            TreeNode::wrapped(1),
            TreeNode::with(3, None, TreeNode::wrapped(1)),
        ),
        TreeNode::with(1, None, None),
    );

    println!("{}", Solution::pseudo_palindromic_paths(tree));
}
