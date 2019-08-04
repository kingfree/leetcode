// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
use leetcode_prelude::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        println!("{:?}", root);
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use leetcode_prelude::btree;

    #[test]
    fn test() {
        assert_eq!(
            Solution::btree_game_winning_move(btree![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 11, 3),
            true
        );
    }

}
