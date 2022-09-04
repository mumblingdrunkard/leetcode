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
// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/

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

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};
type N = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn vertical_traversal(root: N) -> Vec<Vec<i32>> {
        let mut columns = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((root, 0, 0));

        while let Some((node, r, c)) = q.pop_front() {
            if let Some(node) = node {
                columns
                    .entry(c)
                    .or_insert(vec![])
                    .push((r, node.borrow().val));

                q.push_back((node.borrow().left.clone(), r + 1, c - 1));
                q.push_back((node.borrow().right.clone(), r + 1, c + 1));
            }
        }

        let mut res = columns.into_iter().collect::<Vec<_>>();
        res.sort(); // sort so columns appear in correct order
        res.iter_mut().for_each(|(_, v)| v.sort()); // sort so overlapping nodes appear in
                                                    // specified order

        // just map it
        res.into_iter()
            .map(|(_, v)| v.into_iter().map(|(_, val)| val).collect::<Vec<_>>())
            .collect()
    }
}

fn main() {
    let tree = TreeNode::with(
        3,
        TreeNode::wrapped(9),
        TreeNode::with(20, TreeNode::wrapped(15), TreeNode::wrapped(7)),
    );

    println!("{:?}", Solution::vertical_traversal(tree));

    let tree = TreeNode::with(
        1,
        TreeNode::with(2, TreeNode::wrapped(4), TreeNode::wrapped(6)),
        TreeNode::with(3, TreeNode::wrapped(5), TreeNode::wrapped(7)),
    );

    println!("{:?}", Solution::vertical_traversal(tree));
}
