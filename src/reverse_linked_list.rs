use std::clone;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution{}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_node: Option<Box<ListNode>> = head;
        let mut previous_node:Option<Box<ListNode>> = None;

        while let Some(mut node) = current_node.take() {
            let nxt = node.next;
            node.next = previous_node.take();
            previous_node = Some(node);
            current_node = nxt;
            
        }
        previous_node
        
    }
}