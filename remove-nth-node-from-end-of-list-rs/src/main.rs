// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/remove-nth-node-from-end-of-list/

struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: N,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn wrapped(val: i32) -> N {
        Some(Box::new(Self { val, next: None }))
    }

    pub fn with(val: i32, next: N) -> N {
        Some(Box::new(Self { val, next }))
    }
}

type N = Option<Box<ListNode>>;

impl Solution {
    pub fn remove_nth_node_from_end(mut head: N, n: i32) -> N {
        let n = n as usize;

        let len = {
            let mut len = 0;
            let mut curr = &head;
            while let Some(node) = curr {
                curr = &node.next;
                len += 1;
            }
            len
        };

        if n == len {
            return head.unwrap().next.take();
        }

        let target = len - n as usize;
        let mut prev = &mut head;
        for _ in 0..target - 1 {
            if let Some(node) = prev {
                prev = &mut node.next;
            }
        }

        if let Some(node) = prev {
            let mut rest = node.next.take();
            if let Some(mut node) = rest {
                rest = node.next.take();
            }
            node.next = rest;
        }

        head
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::remove_nth_node_from_end(
            ListNode::with(
                1,
                ListNode::with(
                    2,
                    ListNode::with(3, ListNode::with(4, ListNode::wrapped(5)))
                )
            ),
            2
        )
    );
}
