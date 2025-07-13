use super::Solve;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[allow(dead_code)]
    fn from_vec(digits: Vec<i32>) -> Self {
        let mut list = ListNode::new(digits[0]);
        let mut current = &mut list;

        for digit in &digits[1..] {
            current.next = Some(Box::new(ListNode::new(*digit)));
            current = current.next.as_mut().unwrap();
        }
        list
    }
}

/// # 2. Add Two Numbers
///
/// Medium
///
/// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
pub struct AddTwoNumbers;

impl AddTwoNumbers {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = result.as_mut();

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

        while l1.is_some() || l2.is_some() {
            let mut sum = 0;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_ref();
            }

            sum += carry;
            carry = sum / 10;
            sum %= 10;

            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            current = current.unwrap().next.as_mut();
        }

        if carry != 0 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        result.unwrap().next
    }
}

impl Solve<(Option<Box<ListNode>>, Option<Box<ListNode>>), Option<Box<ListNode>>>
for AddTwoNumbers
{
    fn solve(input: (Option<Box<ListNode>>, Option<Box<ListNode>>)) -> Option<Box<ListNode>> {
        Self::add_two_numbers(input.0, input.1)
    }
}

#[test]
fn add_two_numbers_lc1() {
    let l1 = ListNode::from_vec(vec![2, 4, 3]);
    let l2 = ListNode::from_vec(vec![5, 6, 4]);
    let result = AddTwoNumbers::solve((Some(Box::new(l1)), Some(Box::new(l2))));
    let expected = Some(Box::new(ListNode::from_vec(vec![7, 0, 8])));

    assert_eq!(result, expected)
}

#[test]
fn add_two_numbers_lc2() {
    let l1 = ListNode::from_vec(vec![0]);
    let l2 = ListNode::from_vec(vec![0]);
    let result = AddTwoNumbers::solve((Some(Box::new(l1)), Some(Box::new(l2))));
    let expected = Some(Box::new(ListNode::from_vec(vec![0])));

    assert_eq!(result, expected)
}

#[test]
fn add_two_numbers_lc3() {
    let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
    let result = AddTwoNumbers::solve((Some(Box::new(l1)), Some(Box::new(l2))));
    let expected = Some(Box::new(ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1])));

    assert_eq!(result, expected)
}
