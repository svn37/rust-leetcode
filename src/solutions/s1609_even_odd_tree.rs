use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        let mut queue = Vec::new();
        let mut next_queue = Vec::new();
        queue.push(root.unwrap());

        let mut cur_ord = Ordering::Less;
        let mut cur_parity = true; // starts with odd numbers
        loop {
            if !queue
                .iter()
                .all(|n| n.borrow().val & 1 == cur_parity as i32)
                || !queue
                    .windows(2)
                    .all(|w| w[0].borrow().val.cmp(&w[1].borrow().val) == cur_ord)
            {
                return false;
            }

            for node in &queue {
                if let Some(left) = node.borrow().left.clone() {
                    next_queue.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    next_queue.push(right);
                }
            }
            queue.clear();
            if next_queue.is_empty() {
                return true;
            }
            std::mem::swap(&mut queue, &mut next_queue);
            cur_ord = cur_ord.reverse();
            cur_parity = !cur_parity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1609() {
        // TODO add norma tests with binary tree construction
        assert!(Solution::is_even_odd_tree(None))
    }
}