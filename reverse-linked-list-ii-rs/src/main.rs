// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/reverse-linked-list-ii/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    fn new_with_next(val: i32, next: Option<Box<Self>>) -> Self {
        Self { val, next }
    }
}

struct Solution {}

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // allow mutation
        let mut mock = Some(Box::new(ListNode { val: 0, next: head }));
        let mut start = &mut mock;

        for _ in 1..left {
            if let Some(node) = start {
                start = &mut node.next;
            }
        }

        let mut current = Self::take_child(start);
        let mut previous = None;
        for _ in left..right {
            let next = Self::take_child(&mut current);
            Self::give_child(&mut current, previous);
            previous = current;
            current = next;
        }

        let rest = Self::take_child(&mut current);
        Self::give_child(&mut current, previous);
        Self::give_child(start, current);

        // go to new end of list
        for _ in left..right + 1 {
            if let Some(node) = start {
                start = &mut node.next;
            }
        }

        Self::give_child(start, rest);
        Self::take_child(&mut mock)
    }

    pub fn take_child(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_mut()?.next.take()
    }

    pub fn give_child(head: &mut Option<Box<ListNode>>, child: Option<Box<ListNode>>) {
        if let Some(node) = head {
            node.next = child;
        }
    }
}

fn main() {
    let list = Some(Box::new(ListNode::new_with_next(
        1,
        Some(Box::new(ListNode::new_with_next(
            2,
            Some(Box::new(ListNode::new_with_next(
                3,
                Some(Box::new(ListNode::new_with_next(
                    4,
                    Some(Box::new(ListNode::new(5))),
                ))),
            ))),
        ))),
    )));
    println!("{:?}", Solution::reverse_between(list, 2, 4));
}
