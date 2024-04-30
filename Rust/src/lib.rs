// Helper functions for LinkedList
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn append(&mut self, val: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(ListNode::new(val)));
    }
}

pub fn create_linked_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &val in vals.iter() {
        let node = ListNode::new(val);
        if let Some(ref mut tail) = head {
            tail.append(val);
        } else {
            head = Some(Box::new(node));
        }
    }
    head
}

pub mod add_two_numbers;
pub mod best_time2buy_n_sell_stock;
pub mod binary_search;
pub mod climbing_stairs;
pub mod count_odd_number;
pub mod find_index_of_the1st_occur_in_a_string;
pub mod len_of_last_word;
pub mod longest_common_prefix;
pub mod maximum_subarray;
pub mod merge_2_sorted_lists;
pub mod merge_sorted_arr;
pub mod missing_number;
pub mod num_of_good_pairs;
pub mod palindrome_num;
pub mod parking_system;
pub mod two_sum;
