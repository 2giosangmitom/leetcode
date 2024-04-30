#[cfg(test)]
mod tests {
    use leetcode::add_two_numbers::*;
    use leetcode::create_linked_list;

    #[test]
    fn test_add_two_numbers_case1() {
        let l1 = create_linked_list(&[2, 4, 3]);
        let l2 = create_linked_list(&[5, 6, 4]);
        let want = create_linked_list(&[7, 0, 8]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, want);
    }

    #[test]
    fn test_add_two_numbers_case2() {
        let l1 = create_linked_list(&[0]);
        let l2 = create_linked_list(&[0]);
        let want = create_linked_list(&[0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, want);
    }

    #[test]
    fn test_add_two_numbers_case3() {
        let l1 = create_linked_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_linked_list(&[9, 9, 9, 9]);
        let want = create_linked_list(&[8, 9, 9, 9, 0, 0, 0, 1]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, want);
    }
}
