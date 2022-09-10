// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/construct-string-from-binary-tree/

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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
type N = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn tree2str(root: N) -> String {
        if let Some(node) = root {
            let left = Self::tree2str(node.borrow().left.clone());
            let right = Self::tree2str(node.borrow().right.clone());
            let val = node.borrow().val;
            if right.is_empty() && left.is_empty() {
                format!("{val}")
            } else if right.is_empty() {
                format!("{val}({left})")
            } else {
                format!("{val}({left})({right})")
            }
        } else {
            String::new()
        }
    }
}

fn main() {
    let tree = TreeNode::with(
        1,
        TreeNode::with(2, TreeNode::wrapped(4), None),
        TreeNode::wrapped(3),
    );

    println!("{}", Solution::tree2str(tree));
}
