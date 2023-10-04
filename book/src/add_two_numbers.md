# [2. Add Two Numbers](https://leetcode.com/problems/add-two-numbers/description/)

> **Solution**

1. Create a placeholder node named `dummyHead`. This node will hold the result.
2. Initialize a variable named `tail` and assign it to `&mut dummyHead`. This variable will keep track of the last node in the `dummyHead`.
3. Initialize a variable named `carry`, assign it to `0`. This variable will store the carry value during addition.
4. Start a while loop that continues until there are no more digits in both input lists (`l1` and `l2`) and there is no remaining `carry` value.
5. Inside the loop:
   - Check if there is a digit in the current node of `l1`. If it exists, assign its value to a variable called `digit1`. Otherwise, set `digit1` to `0`.
   - Check if there is a digit in the current node of `l2`. If it exists, assign its value to a variable called `digit2`. Otherwise, set `digit2` to `0`.
   - Add the current digits from `l1` and `l2`, along with the `carry` value from the previous iteration, and store the sum in a variable called `sum`.
   - Calculate the unit digit of `sum` by taking the modulus of `sum` by 10. This digit will be placed in a new node for the result.
   - Update the `carry` variable by dividing `sum` by 10 and taking the integer division. This gives us the `carry` value for the next iteration.
   - Create a new node (`new_node`) with the calculated digit as its value.
   - Assign the `new_node` to the `tail.next` node.
   - Move the `tail` reference to the newly added node.
6. Repeat step **5** until one of the above conditions in the while loop is met.
7. Return `dummyHead.next`.

- _Time complexity:_ \\( O(n) \\)
- _Space complexity:_ \\( O(1) \\)

```rust
{{#include ../../src/add_two_numbers.rs}}
```
