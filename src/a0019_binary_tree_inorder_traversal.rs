/*
 * [0019] binary-tree-inorder-traversal
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut curr = root.clone();

        while let Some(cur) = curr.clone() {
            let cur = cur.borrow();
            if cur.left.is_some() {
                let mut pre = cur.left.clone().unwrap();
                while pre.borrow().right.is_some() && pre.borrow().right != curr {
                    let a = pre.borrow().right.clone().unwrap();
                    pre = a;
                }
                if pre.borrow().right.is_none() {
                    pre.borrow_mut().right = curr;
                    curr = cur.left.clone();
                }
                if pre.borrow().right == curr {
                    pre.borrow_mut().right = Option::None;
                    res.push(cur.val);
                    curr = cur.right.clone();
                }
            } else {
                res.push(cur.val);
                curr = cur.right.clone();
            }
        }

        res
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
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
    }
}

// solution tests ends here
