use leetcode_rs::collections::ListNode;
use leetcode_rs::solution::Solution;

fn main() {
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];
    let output = Solution::add_two_numbers(ListNode::from(&l1), ListNode::from(&l2));
    println!("Input: l1 = {:?}, l2 = {:?}", &l1, &l2);
    println!("Output: {:?}", listnode_to_vec(output));
    println!();

    let l1 = vec![0];
    let l2 = vec![0];
    let output = Solution::add_two_numbers(ListNode::from(&l1), ListNode::from(&l2));
    println!("Input: l1 = {:?}, l2 = {:?}", &l1, &l2);
    println!("Output: {:?}", listnode_to_vec(output));
    println!();

    let l1 = vec![9, 9, 9, 9, 9, 9, 9];
    let l2 = vec![9, 9, 9, 9];
    let output = Solution::add_two_numbers(ListNode::from(&l1), ListNode::from(&l2));
    println!("Input: l1 = {:?}, l2 = {:?}", &l1, &l2);
    println!("Output: {:?}", listnode_to_vec(output));
}

fn listnode_to_vec(listnode: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = listnode;
    loop {
        match current {
            Some(node) => {
                vec.push(node.val);
                current = node.next;
            }
            None => {
                break;
            }
        }
    }
    vec
}
