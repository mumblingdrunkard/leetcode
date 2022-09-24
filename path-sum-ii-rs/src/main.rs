// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/path-sum-ii/

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
    pub fn path_sum(root: N, target_sum: i32) -> Vec<Vec<i32>> {
        fn recursive(root: N, target_sum: i32, sum: i32) -> Vec<Vec<i32>> {
            if let Some(node) = root {
                if let (None, None) = (node.borrow().left.clone(), node.borrow().right.clone()) {
                    if node.borrow().val + sum == target_sum {
                        return vec![vec![node.borrow().val]];
                    }
                } else {
                    let mut res = vec![];
                    res.append(&mut recursive(
                        node.borrow().left.clone(),
                        target_sum,
                        node.borrow().val + sum,
                    ));

                    res.append(&mut recursive(
                        node.borrow().right.clone(),
                        target_sum,
                        node.borrow().val + sum,
                    ));

                    res.iter_mut().for_each(|v| v.push(node.borrow().val));
                    return res;
                }
            }

            vec![]
        }

        recursive(root, target_sum, 0)
            .into_iter()
            .map(|v| v.into_iter().rev().collect())
            .collect()
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

    println!("{:?}", Solution::path_sum(tree, 22));
}
