# Construct binary tree from inorder and postorder traversal
# Approach
<!-- Describe your approach to solving the problem. -->

1. Create a HashMap to store the indices of elements in the inorder traversal. This allows us to find the index of any element in the inorder traversal in O(1) time.

---


2. Initialize a variable postorder_idx as the last index of the postorder traversal. We will use this variable to keep track of the current root node as we iterate through the postorder traversal in reverse.

---


3. Create a helper function build_tree_helper that takes the following arguments:

    inorder and postorder traversals
    the HashMap of inorder indices
    the start and end indices for the current subtree in the inorder traversal
    a mutable reference to the postorder_idx

---


4. In the helper function, perform the following steps:

**a.** If the start index is greater than or equal to the end index in the inorder traversal, return None, as there are no nodes left to process in the current subtree.

**b.** Create a new TreeNode with the value at the current postorder_idx in the postorder traversal.

**c.** Find the index of the root node in the inorder traversal using the HashMap.

**d.** Decrement the postorder_idx by 1.

**e.** Recursively call the helper function to construct the right subtree of the root node, passing the updated postorder_idx and the appropriate start and end indices for the inorder traversal.

**f.** Recursively call the helper function to construct the left subtree of the root node, passing the updated postorder_idx and the appropriate start and end indices for the inorder traversal.

**g.** Return the root node.

---

5. Call the helper function with the initial values of the arguments and return the constructed tree.

---


# Complexity
- Time complexity:
<!-- Add your time complexity here, e.g. $$O(n)$$ -->
$$O(n)$$ by using the HashMap to store the inorder indices.
- Space complexity:
<!-- Add your space complexity here, e.g. $$O(n)$$ -->
$$O(n)$$ where n is the number of nodes in the tree.

![Screenshot 2023-03-17 114857](https://user-images.githubusercontent.com/28766618/225791524-9053c1c7-586e-4e7a-ae3c-958f5a59a3d9.png)
