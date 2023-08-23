// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub trait ListNodeTrait {
    fn new(val: i32) -> Self;
}

impl ListNodeTrait for ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Solution start here
pub struct Solution;

pub trait Merge2SortedLists {
    fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

impl Merge2SortedLists for Solution {
    fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current_node = &mut head;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                current_node.next = list1.take();
                current_node = current_node.next.as_mut().unwrap();
                list1 = current_node.next.take();
            } else {
                current_node.next = list2.take();
                current_node = current_node.next.as_mut().unwrap();
                list2 = current_node.next.take();
            }
        }

        current_node.next = list1.or(list2);
        head.next
    }
}
