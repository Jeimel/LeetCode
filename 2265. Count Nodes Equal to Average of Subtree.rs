use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::average(&root).0
    }

    pub fn average(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        let Some(root) = root else {
            return (0, 0, 0);
        };

        let root = root.borrow();

        let left = Self::average(&root.left);
        let right = Self::average(&root.right);

        let sum = left.1 + right.1 + root.val;
        let count = left.2 + right.2 + 1;

        (left.0 + right.0 + i32::from(root.val == sum / count), sum, count)
    }
}
