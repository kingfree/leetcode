// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use leetcode_prelude::ListNode;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let (mut l1_end, mut l2_end) = (false, false);
        let mut ten = 0;

        let mut res = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut res;

        loop {
            let a = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                },
                None => {
                    l1_end = true;
                    0
                },
            };
            let b = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                },
                None => {
                    l2_end = true;
                    0
                },
            };
            if l1_end && l2_end && ten < 1 {
                break res.unwrap().next;
            }
            let c = a + b + ten;
            let c = if c >= 10 {
                ten = 1;
                c - 10
            } else {
                ten = 0;
                c
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(c)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use leetcode_prelude::linkedlist;

    #[test]
    fn test() {
        assert_eq!(Solution::add_two_numbers(
                linkedlist![2, 4, 3],
                linkedlist![5, 6, 4]
            ),
            linkedlist![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(
                linkedlist![9, 9, 9, 9, 9],
                linkedlist![1, 2, 3, 4, 5]
            ),
            linkedlist![0, 2, 3, 4, 5, 1]);
    }

}