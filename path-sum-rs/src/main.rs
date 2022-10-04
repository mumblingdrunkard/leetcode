// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/path-sum/

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

type N = Option<Rc<RefCell<TreeNode>>>;
use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                target_sum - node.borrow().val == 0
            } else {
                Self::has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val)
                    || Self::has_path_sum(
                        node.borrow().right.clone(),
                        target_sum - node.borrow().val,
                    )
            }
        } else {
            false
        }
    }
}

fn main() {
    let tree = TreeNode::with(
        5,
        TreeNode::with(
            4,
            TreeNode::with(11, TreeNode::wrapped(7), TreeNode::wrapped(2)),
            None,
        ),
        TreeNode::with(
            8,
            TreeNode::wrapped(13),
            TreeNode::with(4, TreeNode::wrapped(5), TreeNode::wrapped(1)),
        ),
    );

    println!("{:?}", Solution::has_path_sum(tree, 22));
}
