/*
 * [1315] sum-of-nodes-with-even-valued-grandparent
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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum: i32 = 0;
        Self::dfs(1, 1, root, &mut sum);
        sum
    }

    fn dfs(gval: i32, pval: i32, root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(root) = root {
            let root = root.borrow();
            if gval % 2 == 0 {
                *sum += root.val;
            }

            Self::dfs(pval, root.val, root.left.clone(), sum);
            Self::dfs(pval, root.val, root.right.clone(), sum);
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
            Solution::sum_even_grandparent(tree![
                6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
            ]),
            18
        );
    }
}

// solution tests ends here
