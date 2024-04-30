use crate::ListNode;

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut prev = &mut head;

    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        if node1.val < node2.val {
            prev.next = list1.take();
            prev = prev.next.as_mut().unwrap();
            list1 = prev.next.take();
        } else {
            prev.next = list2.take();
            prev = prev.next.as_mut().unwrap();
            list2 = prev.next.take();
        }
    }

    prev.next = list1.or(list2);
    head.next
}
