use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn left_child(self, node: TreeNode) -> Self {
        TreeNode {
            left: Some(Rc::new(RefCell::new(node))),
            ..self
        }
    }

    pub fn right_child(self, node: TreeNode) -> Self {
        TreeNode {
            right: Some(Rc::new(RefCell::new(node))),
            ..self
        }
    }
}
