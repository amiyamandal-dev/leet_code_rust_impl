// Definition for singly-linked list.
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
}

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut temp = head.as_ref();
        let mut count = 0;
        loop {
            if temp.is_some() {
                count += 1;
                temp = match temp {
                    Some(t) => t.next.as_ref(),
                    None => None,
                };
            } else if temp.is_none() {
                break;
            }
        }
        let mut mid = (count / 2) + 1;
        let mut temp = head;
        loop {
            if mid > 0 {
                temp = match temp {
                    Some(t) => t.next,
                    None => None,
                };
                mid -= 1;
            } else {
                break;
            }
        }

        temp
    }
}
