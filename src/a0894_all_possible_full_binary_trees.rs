/*
 * [0894] all-possible-full-binary-trees
 */

use super::utils::tree::*;
struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut m: HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>> = HashMap::new();
        Self::get(n, &mut m)
    }

    fn get(
        n: i32,
        m: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if !m.contains_key(&n) {
            let mut ans = Vec::new();
            if n == 1 {
                ans.push(Some(Rc::new(RefCell::new(TreeNode {
                    left: Option::None,
                    right: Option::None,
                    val: 0,
                }))));
            } else if n % 2 == 1 {
                for x in 0..n {
                    let y = n - 1 - x;
                    for left in Self::all_possible_fbt(x) {
                        for right in Self::all_possible_fbt(y) {
                            let n = Some(Rc::new(RefCell::new(TreeNode {
                                left: left.clone(),
                                right: right.clone(),
                                val: 0,
                            })));
                            ans.push(n);
                        }
                    }
                }
            }
            m.insert(n, ans);
        }
        m.get(&n).unwrap().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::all_possible_fbt(1), vec![tree![0]]);
        assert_eq!(
            Solution::all_possible_fbt(7),
            vec![
                tree![0, 0, 0, null, null, 0, 0, null, null, 0, 0],
                tree![0, 0, 0, null, null, 0, 0, 0, 0],
                tree![0, 0, 0, 0, 0, 0, 0],
                tree![0, 0, 0, 0, 0, null, null, null, null, 0, 0],
                tree![0, 0, 0, 0, 0, null, null, 0, 0]
            ]
        );
    }
}
