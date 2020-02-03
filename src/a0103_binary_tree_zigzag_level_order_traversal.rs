/*
 * [0103] binary-tree-zigzag-level-order-traversal
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut q = VecDeque::new();
        let mut zigzag = false;
        let mut res = Vec::new();
        q.push_back(root);
        while !q.is_empty() {
            let mut v: Vec<i32> = Vec::new();
            let mut size = q.len();
            while size > 0 {
                if zigzag {
                    let r = q.pop_front().unwrap().unwrap();
                    let b = r.borrow();
                    if !b.right.is_none() {
                        q.push_back(b.right.clone());
                    }
                    if !b.left.is_none() {
                        q.push_back(b.left.clone());
                    }
                    v.push(b.val);
                } else {
                    let r = q.pop_back().unwrap().unwrap();
                    let b = r.borrow();
                    if !b.left.is_none() {
                        q.push_front(b.left.clone());
                    }
                    if !b.right.is_none() {
                        q.push_front(b.right.clone());
                    }
                    v.push(b.val);
                }
                size -= 1;
            }
            zigzag = !zigzag;
            res.push(v);
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
            Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}

// solution tests ends here
