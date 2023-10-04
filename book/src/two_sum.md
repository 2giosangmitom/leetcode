# [1. Two Sum](https://leetcode.com/problems/two-sum/description/)

> **First solution: using a Hash Table**

1. Create an empty **hash table** (`hash_map`) to store elements and their indexes.
2. Iterate through the `nums` from left to right.
3. For each element `nums[i]`, calculate the `complement` by subtracting it from the target: `complement = target - nums[i]`.
4. Check if the `complement` exists in the `hash_map`. If it does, return the value of `complement` stored in `hash_map` and `i`.
5. If the `complement` does not exist in the `hash_map`, add the current element `nums[i]` to the `hash_map` with its index as the value.
6. Repeat steps 3-5 until we find a solution or reach the end of the `nums`.
7. If no solution is found, return an appropriate indicator.

> **Second solution: using 2 nested loops**

1. Iterate through the `nums` from left to right.
2. For each element `nums[i]`, calculate the `complement` by subtracting it from the target: `complement = target - nums[i]`.
3. Iterate through `nums` one more time.
4. For each element `nums[j]`, check if the `complement = nums[j]` and the `i != j`. If it does, return `i` and `j`.
5. Repeat step 1-4 until condition is _true_ or reach the end of the `nums`
6. If no solution is found, return an appropriate indicator.

```rust
{{#include ../../src/two_sum.rs}}
```
