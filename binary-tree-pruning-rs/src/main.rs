// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/binary-tree-pruning/

struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
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

    pub fn wrapped(val: i32) -> Node {
        Some(Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        })))
    }

    pub fn with(val: i32, left: Node, right: Node) -> Node {
        Some(Rc::new(RefCell::new(Self { val, left, right })))
    }
}

use std::{cell::RefCell, rc::Rc};
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn prune_tree(root: Node) -> Node {
        if Self::prune_tree_recursive(root.clone()) {
            root
        } else {
            None
        }
    }

    fn prune_tree_recursive(root: Node) -> bool {
        if let Some(node) = root {
            let l = Self::prune_tree_recursive(node.borrow().left.clone());
            if !l {
                node.borrow_mut().left = None;
            }

            let r = Self::prune_tree_recursive(node.borrow().right.clone());
            if !r {
                node.borrow_mut().right = None;
            }

            l || r || node.borrow().val == 1
        } else {
            false
        }
    }
}

fn main() {
    let tree = TreeNode::with(
        1,
        None,
        TreeNode::with(0, TreeNode::wrapped(0), TreeNode::wrapped(1)),
    );
    println!("{:?}", Solution::prune_tree(tree));
}
