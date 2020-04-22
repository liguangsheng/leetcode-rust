/*
 * [0701] insert-into-a-binary-search-tree
 */

use super::utils::tree::*;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone() {
            let mut root = root.borrow_mut();

            if val < root.val {
                if root.left.is_none() {
                    root.left = Some(Rc::new(RefCell::new(TreeNode {
                        left: Option::None,
                        right: Option::None,
                        val: val,
                    })))
                } else {
                    Self::insert_into_bst(root.left.clone(), val);
                }
            } else {
                if root.right.is_none() {
                    root.right = Some(Rc::new(RefCell::new(TreeNode {
                        left: Option::None,
                        right: Option::None,
                        val: val,
                    })))
                } else {
                    Self::insert_into_bst(root.right.clone(), val);
                }
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::insert_into_bst(tree![4, 2, 7, 1, 3], 5),
            tree![4, 2, 7, 1, 3, 5]
        );
    }
}
