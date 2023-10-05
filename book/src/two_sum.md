# [1. Two Sum](https://leetcode.com/problems/two-sum/description/)

> **First solution: using hash map**

1. Create an empty **hash table** (`hash_map`) to store elements and their indexes.
2. Iterate through the `nums` from left to right.
3. Inside the loop:
   - For each element `nums[i]`, calculate the `complement` by subtracting it from the target: `complement = target - nums[i]`.
   - Check if the `complement` exists in the `hash_map`. If it does, return the value of `complement` stored in `hash_map` and `i`.
   - If the `complement` does not exist in the `hash_map`, add the current element `nums[i]` to the `hash_map` with its index as the value.
4. Repeat step **3** until we find a result or reach the end of the `nums`.
5. If no result is found, return an appropriate indicator.

- _Time complexity:_ \\( O(n) \\)
- _Space complexity:_ \\( O(n) \\)

> **Second solution: using nested loops**

1. Iterate through the `nums` from left to right.
2. Inside the loop:
   - For each element `nums[i]`, calculate the `complement` by subtracting it from the target: `complement = target - nums[i]`.
   - Iterate through `nums` one more time.
   - Inside the second loop:
     - For each element `nums[j]`, check if the `complement = nums[j]` and the `i != j`.
     - If the conditions are true, return `[i, j]`.
3. Repeat step **2** until we find a result or reach the end of the `nums`
4. If no result is found, return an appropriate indicator.

- _Time complexity:_ \\( O({n^2}) \\)
- _Space complexity:_ \\( O(1) \\)

```rust
{{#include ../../src/two_sum.rs}}
```
