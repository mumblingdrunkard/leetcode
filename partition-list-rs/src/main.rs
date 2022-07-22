// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/partition-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn new_with_next(val: i32, next: Option<Box<Self>>) -> Self {
        Self { val, next }
    }
}

struct Solution {}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut before = Some(Box::new(ListNode::new(0)));
        let mut after = Some(Box::new(ListNode::new(0)));

        let mut before_end = &mut before;
        let mut after_end = &mut after;

        let mut current = head;
        loop {
            let next = Self::take_child(&mut current);
            if let Some(node) = &current {
                if node.val < x {
                    Self::give_child(before_end, current);
                    before_end = if let Some(node) = before_end {
                        &mut node.next
                    } else {
                        before_end
                    };
                } else {
                    Self::give_child(after_end, current);
                    after_end = if let Some(node) = after_end {
                        &mut node.next
                    } else {
                        after_end
                    };
                }
            } else {
                break;
            }
            current = next;
        }

        Self::give_child(before_end, Self::take_child(&mut after));
        Self::take_child(&mut before)
    }

    fn take_child(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(node) => node.next.take(),
            _ => None,
        }
    }

    fn give_child(head: &mut Option<Box<ListNode>>, child: Option<Box<ListNode>>) {
        if let Some(node) = head {
            node.next = child;
        }
    }
}

fn main() {
    let list = Some(Box::new(ListNode::new_with_next(
        5,
        Some(Box::new(ListNode::new_with_next(
            4,
            Some(Box::new(ListNode::new_with_next(
                3,
                Some(Box::new(ListNode::new_with_next(
                    2,
                    Some(Box::new(ListNode::new(1))),
                ))),
            ))),
        ))),
    )));

    println!("{:?}", Solution::partition(list, 3));
}
