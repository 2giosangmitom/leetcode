# Solution for LeetCode problems

[![C#](https://github.com/2giosangmitom/leetcode/actions/workflows/c_sharp.yml/badge.svg)](https://github.com/2giosangmitom/leetcode/actions/workflows/c_sharp.yml)
[![Dog](https://github.com/2giosangmitom/leetcode/actions/workflows/go.yml/badge.svg)](https://github.com/2giosangmitom/leetcode/actions/workflows/go.yml)
[![Rust](https://github.com/2giosangmitom/leetcode/actions/workflows/rust.yml/badge.svg)](https://github.com/2giosangmitom/leetcode/actions/workflows/rust.yml)
[![TypeScript](https://github.com/2giosangmitom/leetcode/actions/workflows/typescript.yml/badge.svg)](https://github.com/2giosangmitom/leetcode/actions/workflows/typescript.yml)

## Summary table

| Problem title                                                | Solution                                                                                                                                                               | Difficulty |
| ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| [1. Two Sum][1]                                              | [Rust](./Rust/src/two_sum.rs), [C#](./CSharp/Solutions/TwoSum.cs), [Go](./Golang/two_sum/1.go), [TypeScript](./TypeScript/two_sum/1.ts)                                | Easy       |
| [2. Add Two Numbers][2]                                      | [Rust](./Rust/src/add_two_numbers.rs), [C#](./CSharp/Solutions/AddTwoNumbers.cs), [Go](./Golang/add_two_numbers/2.go), [TypeScript](./TypeScript/add_two_numbers/2.ts) | Medium     |
| [7. Reverse Integer][7]                                      | [Rust](./Rust/src/reverse_integer.rs), [C#](./CSharp/Solutions/ReverseInteger.cs), [Go](./Golang/reverse_integer/7.go)                                                 | Medium     |
| [9. Palindrome Number][9]                                    | [Rust](./Rust/src/palindrome_num.rs), [C#](./CSharp/Solutions/PalindromeNumber.cs), [Go](./Golang/palindrome_number/9.go)                                              | Easy       |
| [13. Roman to Integer][13]                                   | [Rust](./Rust/src/roman2int.rs), [C#](./CSharp/Solutions/RomanToInteger.cs), [Go](./Golang/roman_to_integer/13.go)                                                     | Easy       |
| [14. Longest Common Prefix][14]                              | [Rust](./Rust/src/longest_common_prefix.rs), [C#](./CSharp/Solutions/LongestCommonPrefix.cs), [Go](./Golang/longest_common_prefix/14.go)                               | Easy       |
| [20. Valid Parentheses][20]                                  | [Rust](./Rust/src/valid_parentheses.rs), [Go](./Golang/valid_parentheses/20.go)                                                                                        | Easy       |
| [21. Merge Two Sorted Lists][21]                             | [Rust](./Rust/src/merge_2_sorted_lists.rs), [Go](./Golang/merge_two_sorted_lists/21.go)                                                                                | Easy       |
| [26. Remove Duplicates from Sorted Array][26]                | [Rust](./Rust/src/remove_duplicates_from_sorted_arr.rs), [Go](./Golang/remove_duplicates_from_sorted_array/26.go)                                                      | Easy       |
| [27. Remove Element][27]                                     | [Rust](./Rust/src/remove_element.rs)                                                                                                                                   | Easy       |
| [28. Find the Index of the First Occurrence in a String][28] | [Rust](./Rust/src/find_index_of_the1st_occur_in_a_string.rs)                                                                                                           | Easy       |
| [35. Search Insert Position][35]                             | [Rust](./Rust/src/search_insert_position.rs)                                                                                                                           | Easy       |
| [53. Maximum Subarray][53]                                   | [Rust](./Rust/src/maximum_subarray.rs)                                                                                                                                 | Medium     |
| [58. Length of Last Word][58]                                | [Rust](./Rust/src/len_of_last_word.rs)                                                                                                                                 | Easy       |
| [66. Plus one][66]                                           | [Rust](./Rust/src/plus_one.rs)                                                                                                                                         | Easy       |
| [69. Sqrt(x)][69]                                            | [Rust](./Rust/src/sqrt.rs)                                                                                                                                             | Easy       |
| [70. Climbing Stairs][70]                                    | [Rust](./Rust/src/climbing_stairs.rs)                                                                                                                                  | Easy       |
| [88. Merge Sorted Array][88]                                 | [Rust](./Rust/src/merge_sorted_arr.rs)                                                                                                                                 | Easy       |
| [100. Same Tree][100]                                        | [Rust](./Rust/src/same_tree.rs)                                                                                                                                        | Easy       |
| [121. Best Time to Buy and Sell Stock][121]                  | [Rust](./Rust/src/best_time2buy_n_sell_stock.rs)                                                                                                                       | Easy       |
| [268. Missing Number][268]                                   | [Rust](./Rust/src/missing_number.rs)                                                                                                                                   | Easy       |
| [704. Binary Search][704]                                    | [Rust](./Rust/src/binary_search.rs)                                                                                                                                    | Easy       |
| [929. Unique Email Addresses][929]                           | [Rust](./Rust/src/unique_email_addresses.rs)                                                                                                                           | Easy       |
| [1512. Number of Good Pairs][1512]                           | [Rust](./Rust/src/num_of_good_pairs.rs)                                                                                                                                | Easy       |
| [1518. Water Bottles][1518]                                  | [Rust](./Rust/src/water_bottles.rs)                                                                                                                                    | Easy       |
| [1523. Count Odd Numbers in an Interval Range][1523]         | [Rust](./Rust/src/count_odd_number.rs)                                                                                                                                 | Easy       |
| [1603. Design Parking System][1603]                          | [Rust](./Rust/src/parking_system.rs)                                                                                                                                   | Easy       |

[100]: https://leetcode.com/problems/same-tree/description/
[53]: https://leetcode.com/problems/maximum-subarray/description/
[88]: https://leetcode.com/problems/merge-sorted-array/description/
[1603]: https://leetcode.com/problems/design-parking-system/
[1523]: https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/
[2]: https://leetcode.com/problems/add-two-numbers/
[1518]: https://leetcode.com/problems/water-bottles/
[1512]: https://leetcode.com/problems/number-of-good-pairs/
[28]: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
[1]: https://leetcode.com/problems/two-sum/
[7]: https://leetcode.com/problems/reverse-integer/
[9]: https://leetcode.com/problems/palindrome-number/
[13]: https://leetcode.com/problems/roman-to-integer/
[14]: https://leetcode.com/problems/longest-common-prefix/
[20]: https://leetcode.com/problems/valid-parentheses/
[21]: https://leetcode.com/problems/merge-two-sorted-lists/
[26]: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
[27]: https://leetcode.com/problems/remove-element/
[58]: https://leetcode.com/problems/length-of-last-word/
[69]: https://leetcode.com/problems/sqrtx/
[121]: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
[268]: https://leetcode.com/problems/missing-number/
[704]: https://leetcode.com/problems/binary-search/
[929]: https://leetcode.com/problems/unique-email-addresses/
[66]: https://leetcode.com/problems/plus-one/
[35]: https://leetcode.com/problems/search-insert-position/
[70]: https://leetcode.com/problems/climbing-stairs/
