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

#[test]
fn test_add_two_numbers() {
    use crate::add_two_numbers::ListNode;
    struct Tt {
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        want: Option<Box<ListNode>>,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            l1: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
            l2: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            want: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None })),
                })),
            })),
        },
        Tt {
            l1: Some(Box::new(ListNode { val: 0, next: None })),
            l2: Some(Box::new(ListNode { val: 0, next: None })),
            want: Some(Box::new(ListNode { val: 0, next: None })),
        },
        Tt {
            l1: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode { val: 9, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
            l2: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode { val: 9, next: None })),
                    })),
                })),
            })),
            want: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode { val: 1, next: None })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::add_two_numbers(t.l1, t.l2);
        assert_eq!(result, t.want);
    }
}
