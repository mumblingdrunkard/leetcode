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
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let pn = p.clone().unwrap();
        let qn = q.clone().unwrap();
        let min = std::cmp::min(pn.borrow().val, qn.borrow().val);
        let max = std::cmp::max(pn.borrow().val, qn.borrow().val);

        if let Some(rt) = root.clone() {
            if rt.borrow().val < min {
                Self::lowest_common_ancestor(rt.borrow().right.clone(), p, q)
            } else if rt.borrow().val > max {
                Self::lowest_common_ancestor(rt.borrow().left.clone(), p, q)
            } else {
                root
            }
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
