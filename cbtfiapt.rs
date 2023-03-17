use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_indices: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &val)| (val, i)).collect();
        let mut postorder_idx = postorder.len() - 1;
        Self::build_tree_helper(&inorder, &postorder, &inorder_indices, 0, inorder.len(), &mut postorder_idx)
    }

    fn build_tree_helper(
        inorder: &[i32],
        postorder: &[i32],
        inorder_indices: &HashMap<i32, usize>,
        inorder_start: usize,
        inorder_end: usize,
        postorder_idx: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder_start >= inorder_end {
            return None;
        }

        let root_val = postorder[*postorder_idx];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        // Find the root's index in inorder traversal
        let root_idx = inorder_indices[&root_val];

        // Update the postorder index
        *postorder_idx -= 1;

        // Build the right subtree first since we are iterating the postorder traversal in reverse
        root.borrow_mut().right = Self::build_tree_helper(inorder, postorder, inorder_indices, root_idx + 1, inorder_end, postorder_idx);

        // Build the left subtree
        root.borrow_mut().left = Self::build_tree_helper(inorder, postorder, inorder_indices, inorder_start, root_idx, postorder_idx);

        Some(root)
    }
}