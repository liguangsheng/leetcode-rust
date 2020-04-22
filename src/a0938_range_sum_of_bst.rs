/*
 * [0938] range-sum-of-bst
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        Self::dfs(root, l, r, &mut sum);
        sum
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32, sum: &mut i32) {
        if let Some(root) = root {
            let root = root.borrow();
            if root.val >= l && root.val <= r {
                *sum += root.val;
            }
            if root.val > l {
                Self::dfs(root.left.clone(), l, r, sum);
            }
            if root.val < r {
                Self::dfs(root.right.clone(), l, r, sum);
            }
        }
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
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, null, 18], 7, 15),
            32
        );
        assert_eq!(
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, 13, 18, 1, null, 6], 6, 10),
            23
        );
    }
}

// solution tests ends here
