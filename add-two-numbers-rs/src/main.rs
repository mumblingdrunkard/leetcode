// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/add-two-numbers/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn new_with_tail(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut a, mut b) = (&l1, &l2);

        let mut output = Some(Box::new(ListNode::new(0)));
        let mut current = &mut output;

        while let Some(box_c) = current {
            let curr = box_c.as_mut();
            if let Some(box_a) = a {
                curr.val += box_a.as_ref().val;
                a = &box_a.as_ref().next;
            }

            if let Some(box_b) = b {
                curr.val += box_b.as_ref().val;
                b = &box_b.as_ref().next;
            }

            let carry = curr.val / 10;
            curr.val %= 10;

            if a.is_some() || b.is_some() || carry != 0 {
                curr.next = Some(Box::new(ListNode::new(carry)));
            }

            current = &mut curr.next;
        }

        output
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::add_two_numbers(
            Some(Box::new(ListNode::new_with_tail(
                1,
                Some(Box::new(ListNode::new_with_tail(
                    2,
                    Some(Box::new(ListNode::new(3)))
                )))
            ))),
            Some(Box::new(ListNode::new_with_tail(
                4,
                Some(Box::new(ListNode::new_with_tail(
                    5,
                    Some(Box::new(ListNode::new(7)))
                )))
            )))
        )
    );
}
