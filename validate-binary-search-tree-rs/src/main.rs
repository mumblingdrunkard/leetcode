// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/validate-binary-search-tree/

struct Solution {}

type N = Option<Rc<RefCell<TreeNode>>>;

// Definition for a binary tree node.
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

    pub fn with_children(val: i32, left: N, right: N) -> Self {
        Self { val, left, right }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_internal(root, i64::MIN, i64::MAX)
    }

    fn is_valid_bst_internal(root: Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
        if let Some(rt) = root {
            let parent = rt.borrow();
            let val = parent.val as i64;
            val > lower
                && val < upper
                && Self::is_valid_bst_internal(parent.left.clone(), lower, val)
                && Self::is_valid_bst_internal(parent.right.clone(), val, upper)
        } else {
            true
        }
    }
}

fn main() {
    let seven = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let four = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let two = Some(Rc::new(RefCell::new(TreeNode::with_children(
        2, seven, four,
    ))));

    let six = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let five = Some(Rc::new(RefCell::new(TreeNode::with_children(5, six, two))));

    let zero = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let eight = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    let one = Some(Rc::new(RefCell::new(TreeNode::with_children(
        1, zero, eight,
    ))));

    let three = Some(Rc::new(RefCell::new(TreeNode::with_children(3, five, one))));

    println!("{}", Solution::is_valid_bst(three));
}
