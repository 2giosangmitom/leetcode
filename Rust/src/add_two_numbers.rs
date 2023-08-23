#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

pub trait AddTwoNumbers {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

impl AddTwoNumbers for Solution {
    fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_head;
        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let digit1 = match l1 {
                Some(l) => {
                    l1 = l.next;
                    l.val
                }
                None => 0,
            };

            let digit2 = match l2 {
                Some(l) => {
                    l2 = l.next;
                    l.val
                }
                None => 0,
            };

            let sum = digit1 + digit2 + carry;
            let digit = sum % 10;
            carry = sum / 10;

            let new_node = Some(Box::new(ListNode::new(digit)));
            tail.next = new_node;
            tail = tail.next.as_mut()?;
        }
        dummy_head.next
    }
}
