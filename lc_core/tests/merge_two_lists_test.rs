use lc_core::solutions::{Solve, merge_two_lists::MergeTwoLists as S};
use lc_core::solutions::merge_two_lists::{create_linked_list, ListNode};

// Helper function to verify that a linked list matches expected values
fn verify_linked_list(list: Option<Box<ListNode>>, expected: &[i32]) {
    let mut current = list;
    let mut index = 0;

    while let Some(node) = current {
        assert_eq!(node.val, expected[index], "Value mismatch at index {}", index);
        current = node.next;
        index += 1;
    }

    assert_eq!(index, expected.len(), "List length doesn't match expected length");
}

#[test]
fn test_1() {
    let list_1_vals = [1, 2, 4];
    let list_2_vals = [1, 3, 4];

    let list_1 = create_linked_list(&list_1_vals);
    let list_2 = create_linked_list(&list_2_vals);

    let result = S::solve((list_1, list_2));

    let expected = [1, 1, 2, 3, 4, 4];
    verify_linked_list(result, &expected);
}

#[test]
fn test_2() {
    let list_1_vals = [];
    let list_2_vals = [];

    let list_1 = create_linked_list(&list_1_vals);
    let list_2 = create_linked_list(&list_2_vals);

    let result = S::solve((list_1, list_2));

    let expected = [];
    verify_linked_list(result, &expected);
}

#[test]
fn test_3() {
    let list_1_vals = [];
    let list_2_vals = [0];

    let list_1 = create_linked_list(&list_1_vals);
    let list_2 = create_linked_list(&list_2_vals);

    let result = S::solve((list_1, list_2));

    let expected = [0];
    verify_linked_list(result, &expected);
}