#[cfg(test)]
mod tests {
    use leetcode::create_linked_list;
    use leetcode::merge_2_sorted_lists::*;

    #[test]
    fn test_merge_two_sorted_lists() {
        let list1 = create_linked_list(&[1, 2, 4]);
        let list2 = create_linked_list(&[1, 3, 4]);

        let result = merge_two_lists(list1, list2);

        let want = create_linked_list(&[1, 1, 2, 3, 4, 4]);

        assert_eq!(&result, &want);
    }
}
