// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/flatten-binary-tree-to-linked-list/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: N,
    pub right: N,
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

    pub fn with_children(val: i32, left: N, right: N) -> Self {
        Self { val, left, right }
    }
}

struct Solution {}
type N = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn flatten(root: &mut N) {
        if let Some(rt) = root {
            let mut l = rt.borrow_mut().left.take();
            let mut r = rt.borrow_mut().right.take();
            Self::flatten(&mut l);
            Self::flatten(&mut r);
            rt.borrow_mut().right = l;
            let mut insertion_point = root.clone();
            while let Some(ip) = insertion_point {
                if ip.borrow_mut().right.is_none() {
                    ip.borrow_mut().right = r.take();
                    break;
                } else {
                    insertion_point = ip.borrow_mut().right.clone();
                }
            }
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

    let mut three = Some(Rc::new(RefCell::new(TreeNode::with_children(3, five, one))));

    Solution::flatten(&mut three);

    println!("{:?}", three);
}
