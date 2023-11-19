#[derive(PartialEq, Eq, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

trait ListNodeTrait {
    fn new(val: i32) -> Self;
}

impl ListNodeTrait for ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

trait Merge2SortedLists {
    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

impl Merge2SortedLists for Solution {
    fn merge_two_lists(
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
}

#[test]
fn test_merge_two_sorted_lists() {
    let list1 = Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    });

    let list2 = Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    });

    let result = Solution::merge_two_lists(Some(list1), Some(list2));
    let want = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(&result, &want);
}
