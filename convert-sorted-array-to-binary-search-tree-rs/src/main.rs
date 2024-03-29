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
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

struct Solution {}

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_array_to_bst_internal(&nums)
    }

    fn sorted_array_to_bst_internal(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
        } else if nums.len() == 0 {
            return None;
        }

        let mid = nums.len() / 2;
        let mut parent = TreeNode::new(nums[mid]);
        parent.left = Self::sorted_array_to_bst_internal(&nums[..mid]);
        parent.right = Self::sorted_array_to_bst_internal(&nums[mid + 1..]);

        Some(Rc::new(RefCell::new(parent)))
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
    );
}
