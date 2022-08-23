// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/palindrome-linked-list/

#[allow(unused)]
struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[allow(unused)]
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    fn with_child(val: i32, next: Option<Box<Self>>) -> Option<Box<Self>> {
        Some(Box::new(Self { val, next }))
    }
}

impl Solution {
    // optimal O(n) time, O(1) space. Sucks
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // scan the list to get the length
        let len = {
            let mut r = &head;
            let mut n = 0;
            while let Some(lns) = r {
                r = &lns.next;
                n += 1;
            }
            n
        };

        let mid = len / 2;

        // now mutate the list so that it points outwards in the middle
        let mut prev = None; // becomes the left side, middle-out
        for _ in 0..mid {
            if let Some(mut ln) = head {
                let next = ln.next.take();
                ln.next = prev;
                prev = Some(ln);
                head = next;
            }
        }

        if len % 2 != 0 {
            // advance the right side by 1 so that we don't count the middle
            if let Some(ln) = head {
                head = ln.next;
            }
        }

        prev == head
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_palindrome(ListNode::with_child(
            1,
            ListNode::with_child(
                2,
                ListNode::with_child(1, ListNode::with_child(2, ListNode::with_child(1, None)))
            )
        ))
    );

    println!(
        "{}",
        Solution::is_palindrome(ListNode::with_child(
            1,
            ListNode::with_child(
                2,
                ListNode::with_child(3, ListNode::with_child(2, ListNode::with_child(1, None)))
            )
        ))
    );
}
