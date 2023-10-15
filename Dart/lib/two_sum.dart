import 'dart:collection';

class Solution {
  List<int> twoSum(List<int> nums, int target) {
    final map = HashMap<int, int>();
    for (var i = 0; i < nums.length; i++) {
      int complement = target - nums[i];
      if (map.containsKey(complement)) {
        return [map[complement]!, i];
      }
      map[nums[i]] = i;
    }
    return [-1];
  }

  List<int> twoSum2(List<int> nums, int target) {
    for (var i = 0; i < nums.length; i++) {
      int complement = target - nums[i];
      for (var j = i + 1; j < nums.length; j++) {
        if (nums[j] == complement) {
          return [i, j];
        }
      }
    }
    return [-1];
  }
}
