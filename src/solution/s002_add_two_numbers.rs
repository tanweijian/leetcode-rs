use crate::collections::ListNode;
use crate::solution::Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut tail = &mut dummy_head;
        let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
        loop {
            let lhs = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };
            // if l1, l2 end and there is not overflow from previous operation, return the result
            if l1_end && l2_end && !overflow {
                break dummy_head.unwrap().next;
            }
            let sum = lhs + rhs + if overflow { 1 } else { 0 };
            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            } else {
                overflow = false;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: sum,
                next: None,
            }));
            tail = &mut tail.as_mut().unwrap().next
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::ListNode;
    use crate::solution::Solution;

    #[test]
    fn s002() {
        assert_eq!(
            ListNode::from(vec![7, 0, 8]),
            Solution::add_two_numbers(ListNode::from(vec![2, 4, 3]), ListNode::from(vec![5, 6, 4]))
        );
        assert_eq!(
            ListNode::from(vec![0]),
            Solution::add_two_numbers(ListNode::from(vec![0]), ListNode::from(vec![0]))
        );
        assert_eq!(
            ListNode::from(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            Solution::add_two_numbers(
                ListNode::from(vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::from(vec![9, 9, 9, 9])
            )
        );
    }
}
