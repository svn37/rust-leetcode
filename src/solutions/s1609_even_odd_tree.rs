// A binary tree is named Even-Odd if it meets the following conditions:
//
// 	The root of the binary tree is at level index 0, its children are at level index 1, their children are at level index 2, etc.
// 	For every even-indexed level, all nodes at the level have odd integer values in strictly increasing order (from left to right).
// 	For every odd-indexed level, all nodes at the level have even integer values in strictly decreasing order (from left to right).
//
// Given the root of a binary tree, return true if the binary tree is Even-Odd, otherwise return false.
//
// Example 1:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/sample_1_1966.png" style="width: 362px; height: 229px;" />
//
// Input: root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
// Output: true
// Explanation: The node values on each level are:
// Level 0: [1]
// Level 1: [10,4]
// Level 2: [3,7,9]
// Level 3: [12,8,6,2]
// Since levels 0 and 2 are all odd and increasing, and levels 1 and 3 are all even and decreasing, the tree is Even-Odd.
//
// Example 2:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/sample_2_1966.png" style="width: 363px; height: 167px;" />
//
// Input: root = [5,4,2,3,3,7]
// Output: false
// Explanation: The node values on each level are:
// Level 0: [5]
// Level 1: [4,2]
// Level 2: [3,3,7]
// Node values in the level 2 must be in strictly increasing order, so the tree is not Even-Odd.
//
// Example 3:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/22/sample_1_333_1966.png" style="width: 363px; height: 167px;" />
//
// Input: root = [5,9,1,3,5,7]
// Output: false
// Explanation: Node values in the level 1 should be even integers.
//
// Example 4:
//
// Input: root = [1]
// Output: true
//
// Example 5:
//
// Input: root = [11,8,6,1,3,9,11,30,20,18,16,12,10,4,2,17]
// Output: true
//
//
// Constraints:
//
// 	The number of nodes in the tree is in the range [1, 10^5].
// 	1 <= Node.val <= 10^6
//
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
    use crate::solutions::tests::*;

    #[test]
    fn test_1609() {
        assert_eq!(Solution::is_even_odd_tree(None), false);
        assert_eq!(
            Solution::is_even_odd_tree(to_tree(vec![
                Some(1),
                Some(10),
                Some(4),
                Some(3),
                None,
                Some(7),
                Some(9),
                Some(12),
                Some(8),
                Some(6),
                None,
                None,
                Some(2)
            ])),
            true
        );
        assert_eq!(
            Solution::is_even_odd_tree(to_tree(vec![
                Some(5),
                Some(4),
                Some(2),
                Some(3),
                Some(3),
                Some(7),
            ])),
            false
        );
        assert_eq!(Solution::is_even_odd_tree(to_tree(vec![Some(1)])), true)
    }
}
