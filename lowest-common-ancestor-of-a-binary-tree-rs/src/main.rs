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
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;

type N = Option<Rc<RefCell<TreeNode>>>;

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

struct Solution {}

impl Solution {
    pub fn lowest_common_ancestor(root: N, p: N, q: N) -> N {
        let mut root = root;
        Self::solve_internal(&mut root, &p, &q)
    }

    fn solve_internal(root: &mut N, p: &N, q: &N) -> N {
        if *root == *p || *root == *q {
            return root.take();
        }

        if let Some(inner_root) = root {
            let mut l = Self::solve_internal(&mut inner_root.borrow_mut().left.take(), p, q);
            if let Some(_) = l {
                let r = Self::solve_internal(&mut inner_root.borrow_mut().right.take(), p, q);
                if r.is_some() {
                    return root.take()
                } else {
                    return l.take()
                }
            } else {
                return Self::solve_internal(&mut inner_root.borrow_mut().right.take(), p, q)
            }
        }

        None
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
    let five_ref = five.clone();

    let zero = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let eight = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    let one = Some(Rc::new(RefCell::new(TreeNode::with_children(
        1, zero, eight,
    ))));
    let one_ref = one.clone();

    let three = Some(Rc::new(RefCell::new(TreeNode::with_children(3, five, one))));

    println!(
        "{:?}",
        Solution::lowest_common_ancestor(three, five_ref, one_ref)
    );
}
