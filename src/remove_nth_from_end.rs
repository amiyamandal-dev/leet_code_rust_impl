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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
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

        let new_count = count - n;
        let mut p = temp_head.as_mut();
        for _ in 0..new_count {
            p = p.unwrap().next.as_mut();
        }
        let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        p.as_mut().unwrap().next = next;
        temp_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode { val: 2, next: None })),
                    })),
                })),
            })),
        }));
        let r = Solution::remove_nth_from_end(l, 1);
        print!("{:?}", r);
    }
}
