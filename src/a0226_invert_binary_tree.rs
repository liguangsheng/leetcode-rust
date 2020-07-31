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
    // 使用clone
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

    // 使用unsafe
    pub fn invert_tree_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let ref_mut = root.as_mut().unwrap().as_ptr();
        unsafe {
            let l = &mut (*ref_mut).left;
            let r = &mut (*ref_mut).right;
            std::mem::swap(l, r);
            if l.is_some() {
                Self::invert_tree(Some(l.as_mut().unwrap().clone()));
            }
            if r.is_some() {
                Self::invert_tree(Some(r.as_mut().unwrap().clone()));
            }
        }

        root
    }

    // 使用take
    pub fn invert_tree_3(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root = root.unwrap();
        let l = Self::invert_tree(root.borrow_mut().left.take());
        let r = Self::invert_tree(root.borrow_mut().right.take());
        root.borrow_mut().left = r;
        root.borrow_mut().right = l;
        Some(root)
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }

    #[bench]
    fn bench_invert_tree(b: &mut Bencher) {
        let t = tree![
            4, 1, 4, 6, 8, 9, 5, 3, 1, 3, 5, 6, 7, 8, 9, 7, 65, 3, 2, 2, 3, 4, 45, 6, 6, 78, 9, 0,
            8, 7, 6, 67, 8, 9, 9, 7, 5654, 43, 3, 2, 1, 1, 2, 3, 4, 4, 45, 5, 6, 6, 7, 78, 8, 76,
            5, 4, 67, 8, 9, 90, 009, 7, 65, 87, 654, 356, 789, 87, 6543, 4, 567, 89, 765, 4, 567,
            89, 87, 654, 35, 6789, 0, 876, 543, 4, 5678, 90, 876, 5432, 1, 345, 67890, 9876, 54321,
            2, 345, 6789, 87, 65432, 1, 234, 5678, 2, 7, 1, 3, 6, 9
        ];
        b.iter(|| Solution::invert_tree(t.clone()));
    }

    #[bench]
    fn bench_invert_tree_2(b: &mut Bencher) {
        let t = tree![
            4, 1, 4, 6, 8, 9, 5, 3, 1, 3, 5, 6, 7, 8, 9, 7, 65, 3, 2, 2, 3, 4, 45, 6, 6, 78, 9, 0,
            8, 7, 6, 67, 8, 9, 9, 7, 5654, 43, 3, 2, 1, 1, 2, 3, 4, 4, 45, 5, 6, 6, 7, 78, 8, 76,
            5, 4, 67, 8, 9, 90, 009, 7, 65, 87, 654, 356, 789, 87, 6543, 4, 567, 89, 765, 4, 567,
            89, 87, 654, 35, 6789, 0, 876, 543, 4, 5678, 90, 876, 5432, 1, 345, 67890, 9876, 54321,
            2, 345, 6789, 87, 65432, 1, 234, 5678, 2, 7, 1, 3, 6, 9
        ];
        b.iter(|| Solution::invert_tree_2(t.clone()));
    }

    #[bench]
    fn bench_invert_tree_3(b: &mut Bencher) {
        let t = tree![
            4, 1, 4, 6, 8, 9, 5, 3, 1, 3, 5, 6, 7, 8, 9, 7, 65, 3, 2, 2, 3, 4, 45, 6, 6, 78, 9, 0,
            8, 7, 6, 67, 8, 9, 9, 7, 5654, 43, 3, 2, 1, 1, 2, 3, 4, 4, 45, 5, 6, 6, 7, 78, 8, 76,
            5, 4, 67, 8, 9, 90, 009, 7, 65, 87, 654, 356, 789, 87, 6543, 4, 567, 89, 765, 4, 567,
            89, 87, 654, 35, 6789, 0, 876, 543, 4, 5678, 90, 876, 5432, 1, 345, 67890, 9876, 54321,
            2, 345, 6789, 87, 65432, 1, 234, 5678, 2, 7, 1, 3, 6, 9
        ];
        b.iter(|| Solution::invert_tree_3(t.clone()));
    }
}

// solution tests ends here
