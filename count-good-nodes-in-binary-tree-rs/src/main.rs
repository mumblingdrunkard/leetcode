// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/count-good-nodes-in-binary-tree/

use std::{cell::RefCell, rc::Rc};

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

    pub fn new_wrapped(val: i32) -> N {
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

impl Solution {
    pub fn good_nodes(root: N) -> i32 {
        Self::good_nodes_recursive(root, i32::MIN)
    }

    fn good_nodes_recursive(root: N, max: i32) -> i32 {
        if let Some(node) = root {
            let good = if node.borrow().val >= max { 1 } else { 0 };
            let new_max = std::cmp::max(max, node.borrow().val);
            return good
                + Self::good_nodes_recursive(node.borrow().left.clone(), new_max)
                + Self::good_nodes_recursive(node.borrow().right.clone(), new_max);
        }

        0
    }
}

fn main() {
    let tree = TreeNode::with(
        3,
        TreeNode::with(1, TreeNode::new_wrapped(3), None),
        TreeNode::with(4, TreeNode::new_wrapped(1), TreeNode::new_wrapped(5)),
    );
    println!("{}", Solution::good_nodes(tree));
}
