/*
 * [0617] merge-two-binary-trees
 */

pub struct Solution {}
use super::utils::tree::TreeNode;

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
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_none() {
            return t2;
        }
        if t2.is_none() {
            return t1;
        }

        let b1 = t1.unwrap();
        let b1 = b1.borrow();
        let b2 = t2.unwrap();
        let b2 = b2.borrow();
        Some(Rc::new(RefCell::new(TreeNode {
            val: b1.val + b2.val,
            left: Solution::merge_trees(b1.left.clone(), b2.left.clone()),
            right: Solution::merge_trees(b1.right.clone(), b2.right.clone()),
        })))
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::merge_trees(tree![1], tree![2]), tree![3]);
        assert_eq!(
            Solution::merge_trees(tree![1, 3, 2, 5], tree![2, 1, 3, null, 4, null, 7]),
            tree![3, 4, 5, 5, 4, null, 7]
        );
    }
}

// solution tests ends here
