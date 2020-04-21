/*
 * [0654] maximum-binary-tree
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return Option::None;
        }

        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] > nums[i] {
                i = j;
            }
        }

        let left = Self::construct_maximum_binary_tree(nums[..i].to_vec());
        let right = Self::construct_maximum_binary_tree(nums[i + 1..].to_vec());

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[i],
            left: left,
            right: right,
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
        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
            tree![6, 3, 5, null, 2, 0, null, null, 1]
        );
    }
}

// solution tests ends here
