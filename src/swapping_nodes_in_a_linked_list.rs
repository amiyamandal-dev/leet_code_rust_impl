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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut temp_head = Some(Box::new(ListNode {
            val: -999,
            next: head,
        }));

        let mut count = 0;
        let mut p = temp_head.as_ref();
        while p.unwrap().next.is_some() {
            count += 1;
            p = p.unwrap().next.as_ref();
        }

        let mut s1 = k.min(count - k + 1);
        let mut s2 = count - k;

        let mut p = &mut temp_head;
        for _ in 1..s1 + 1 {
            p = &mut p.as_mut().unwrap().next;
        }

        let s1_val = p.as_ref().unwrap().val;

        for _ in s1..s2 + 1 {
            p = &mut p.as_mut().unwrap().next;
        }

        let s2_val = p.as_mut().unwrap().val;
        p.as_mut().unwrap().val = s1_val;

        let mut p = &mut temp_head;
        for _ in 1..s1 + 1 {
            p = &mut p.as_mut().unwrap().next;
        }
        p.as_mut().unwrap().val = s2_val;

        temp_head.unwrap().next
    }
}
