pub mod s0046_permutations;
pub mod s0456_132_pattern;
pub mod s1609_even_odd_tree;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::utils::binary_tree::TreeNode;

    // TODO values is expected to be valid (i.e. corresponding to correct binary tree and containing
    // the correct number of nulls where needed)
    pub fn to_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let mut nodes: Vec<_> = values.iter().map(|v| create_node(*v)).collect();
        let mut idx = 0;

        for children in nodes[1..].chunks(2) {
            let parent = nodes.get(idx).unwrap();

            if children.len() > 0 {
                if let Some(parent_node) = parent {
                    parent_node.borrow_mut().left = children.get(0).unwrap().clone();
                }
            }

            if children.len() > 1 {
                if let Some(parent_node) = parent {
                    parent_node.borrow_mut().right = children.get(1).unwrap().clone();
                }
            }
            idx += 1;
        }
        nodes.remove(0)
    }

    fn create_node(value: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        value.map(|v| Rc::new(RefCell::new(TreeNode::new(v))))
    }

    #[test]
    fn test_to_tree() {
        assert_eq!(
            to_tree(vec![Some(1), Some(2), Some(3), Some(4)]),
            Some(Rc::new(RefCell::new(
                TreeNode::new(1)
                    .left_child(TreeNode::new(2).left_child(TreeNode::new(4)))
                    .right_child(TreeNode::new(3))
            )))
        );

        assert_eq!(
            to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                None,
                Some(3),
                Some(4),
                None,
                None,
                None,
                None,
                Some(5)
            ]),
            Some(Rc::new(RefCell::new(
                TreeNode::new(1).right_child(
                    TreeNode::new(2)
                        .left_child(TreeNode::new(3).left_child(TreeNode::new(5)))
                        .right_child(TreeNode::new(4))
                )
            )))
        );
    }
}
