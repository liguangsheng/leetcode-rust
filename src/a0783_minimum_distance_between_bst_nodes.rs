/*
 * [0783] minimum-distance-between-bst-nodes
 */

use super::utils::tree::*;
struct Solution;

use std::cell::RefCell;
use std::i32::{MAX, MIN};
use std::rc::Rc;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diff = MAX;
        let mut pval = MIN;
        Self::dfs(root, &mut pval, &mut diff);
        diff
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, pval: &mut i32, diff: &mut i32) {
        if let Some(root) = root {
            let root = root.borrow();
            Self::dfs(root.left.clone(), pval, diff);
            if *pval > MIN {
                *diff = std::cmp::min(*diff, root.val - *pval);
            }
            *pval = root.val;
            Self::dfs(root.right.clone(), pval, diff);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::min_diff_in_bst(tree![4, 2, 6, 1, 3, null, null]),
            1
        );
    }
}
