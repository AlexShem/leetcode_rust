use super::Solve;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct MergeTwoLists;

impl MergeTwoLists {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(l1), Some(l2)) => {
                if l1.val >= l2.val {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: MergeTwoLists::merge_two_lists(Some(l1), l2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: MergeTwoLists::merge_two_lists(l1.next, Some(l2)),
                    }))
                }
            }
        }
    }
}

impl Solve<(Option<Box<ListNode>>, Option<Box<ListNode>>), Option<Box<ListNode>>> for MergeTwoLists {
    fn solve(input: (Option<Box<ListNode>>, Option<Box<ListNode>>)) -> Option<Box<ListNode>> {
        Self::merge_two_lists(input.0, input.1)
    }
}

// Helper function to create a linked list from an array of values
pub fn create_linked_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut list = None;
    for &val in values.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = list;
        list = Some(node);
    }
    list
}