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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        Solution::merge_k_lists_internal(&mut lists)
    }

    pub fn merge_k_lists_internal(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }

        if lists.len() == 1 {
            return lists[0].take();
        }

        if lists.len() == 2 {
            let mut a = lists[0].take();
            let mut b = lists[1].take();
            let mut false_head = Some(Box::new(ListNode::new(0)));
            let mut insertion_point = &mut false_head;

            // merge the two lists
            loop {
                match (&a, &b) {
                    (None, None) => break,
                    (None, Some(_)) => {
                        let next = Solution::take_child(&mut b);
                        Solution::give_child(insertion_point, b);
                        b = next;
                        insertion_point = Solution::advance(insertion_point);
                    }
                    (Some(_), None) => {
                        let next = Solution::take_child(&mut a);
                        Solution::give_child(insertion_point, a);
                        a = next;
                        insertion_point = Solution::advance(insertion_point);
                    }
                    (Some(box_a), Some(box_b)) => {
                        if box_a.val < box_b.val {
                            let next = Solution::take_child(&mut a);
                            Solution::give_child(insertion_point, a);
                            a = next;
                            insertion_point = Solution::advance(insertion_point);
                        } else {
                            let next = Solution::take_child(&mut b);
                            Solution::give_child(insertion_point, b);
                            b = next;
                            insertion_point = Solution::advance(insertion_point);
                        }
                    }
                }
            }

            return Solution::take_child(&mut false_head);
        }

        let half = lists.len()/2;

        Self::merge_k_lists_internal(&mut [
            Self::merge_k_lists_internal(&mut lists[..half]),
            Self::merge_k_lists_internal(&mut lists[half..]),
        ])
    }

    pub fn take_child(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_mut()?.next.take()
    }

    pub fn give_child(head: &mut Option<Box<ListNode>>, child: Option<Box<ListNode>>) {
        if let Some(node) = head {
            node.next = child;
        }
    }

    pub fn advance(reference: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        match reference {
            Some(boxed) => &mut boxed.next,
            _ => reference,
        }
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode::new_with_next(
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
    let list2 = Some(Box::new(ListNode::new_with_next(
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
    let list3 = Some(Box::new(ListNode::new_with_next(
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
    println!("{:?}", Solution::merge_k_lists(vec![list1, list2, list3]));
}
