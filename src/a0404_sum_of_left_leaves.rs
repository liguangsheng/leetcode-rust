/*
 * [0404] sum-of-left-leaves
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(root, false)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, left: bool) -> i32 {
        if let Some(v) = root {
            let r = v.borrow();
            if left && r.left.is_none() && r.right.is_none() {
                return r.val;
            }
            return Solution::helper(r.left.clone(), true)
                + Solution::helper(r.right.clone(), false);
        }
        0
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        let tree = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(Solution::sum_of_left_leaves(tree), 24i32);
    }
}

// solution tests ends here
