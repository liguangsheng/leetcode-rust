/*
 * [0226] invert-binary-tree
 */

use super::utils::tree::*;
pub struct Solution {}

// solution impl starts here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let mut root_mut = root.borrow_mut();

            let temp = root_mut.left.clone();
            root_mut.left = root_mut.right.clone();
            root_mut.right = temp;

            Self::invert_tree(root_mut.left.clone());
            Self::invert_tree(root_mut.right.clone());

            drop(root_mut);
            return Some(root);
        }
        Option::None
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}

// solution tests ends here
