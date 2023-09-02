// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(t) => {
                let t = t.borrow();
                return Solution::is_mirror(t.left.clone(), t.right.clone());
            }
            None => return true,
        }
    }

    pub fn is_mirror(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => return true,
            (Some(left_t), Some(right_t)) => {
                let l = left_t.borrow();
                let r = right_t.borrow();
                if l.val == r.val {
                    let t1 = Solution::is_mirror(l.left.clone(), r.right.clone());
                    let t2 = Solution::is_mirror(l.right.clone(), r.left.clone());
                    return t2 && t1;
                }
                return false;
            }
            _ => return false,
        }
    }
}
