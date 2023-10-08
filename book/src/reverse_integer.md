# [7. Reverse Integer](https://leetcode.com/problems/reverse-integer/description/)

> **Solution description**

1. Declare a variable of type `i32` named `result` and value `0`.
2. Start a `while` loop with the condition `x != 0`.
3. Inside the loop:
   - Check for integer overflow when multiplying `result` by `10` (using `checked_mul` method).
   - If integer overflow occur (`checked_mul` returning `None`), return `0`.
   - If not:
     - Calculate the last digit (`last_digit`) of `x` by dividing `x` by `10` and getting the remainder (`%`).
     - Check for integer overflow when adding `num` to `last_digit` (using `checked_add` method).
     - Return `0` if the integer overflow occurred, else assign the returned value of `checked_add` method to `result`.
   - Remove the last digit of `x` by dividing `x` by `10`.
4. Repeat step **3** until the condition in `while` loop equal to `false`.
5. Return `result`.

- _Time complexity:_ \\( O(n) \\)
- _Space complexity:_ \\( O(1) \\)

```rust
{{#include ../../src/reverse_integer.rs}}
```
