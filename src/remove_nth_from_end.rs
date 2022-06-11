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
        let mut final_list = head.clone();
        let mut fast = final_list.as_mut();
        for _ in 0..n {
            fast = match fast {
                Some(t) => Some(t),
                None => {
                    return match head {
                        Some(t_2) => t_2.next,
                        None => None,
                    }
                }
            };
        }
        fast = match fast {
            Some(t) => {
                let r = match t.next.as_mut() {
                    Some(t_2) => Some(t_2),
                    None => None,
                };
                r
            }
            None => None,
        };

        final_list
    }

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut final_list = head.clone();
        let mut fast = final_list.as_mut();
        while fast.is_some() {
            fast = match fast {
                Some(t) => {
                    if t.val == val {
                        let r = match t.next.as_mut() {
                            Some(t_2) => Some(t_2),
                            None => None,
                        };
                        r
                    } else {
                        Some(t)
                    }
                }
                None => None,
            };
        }

        final_list
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
        let r = Solution::remove_elements(l, 1);
        print!("{:?}", r);
    }
}
