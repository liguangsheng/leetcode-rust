/*
 * [1302] deepest-leaves-sum
 */

use super::utils::tree::*;
pub struct Solution;

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

struct _Solution {
    max_dep: i32,
    sum: i32,
}

impl _Solution {
    fn dfs(&mut self, node: Option<&Rc<RefCell<TreeNode>>>, dep: i32) {
        if let Some(node) = node {
            let node = node.borrow();
            if dep > self.max_dep {
                self.max_dep = dep;
                self.sum = node.val;
            } else if dep == self.max_dep {
                self.sum += node.val;
            }
            self.dfs(node.left.as_ref(), dep + 1);
            self.dfs(node.right.as_ref(), dep + 1);
        }
    }
}

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = _Solution { max_dep: 0, sum: 0 };
        s.dfs(root.as_ref(), 0);
        s.sum
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
            Solution::deepest_leaves_sum(tree![
                1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8
            ]),
            15
        );
    }
}

// solution tests ends here
