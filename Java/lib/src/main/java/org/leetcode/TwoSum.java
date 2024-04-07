package org.leetcode;

import java.util.HashMap;

public class TwoSum {
    public int[] twoSum(int[] nums, int target) {
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
        for (int i = 0; i < nums.length; i++) {
            int neededNumber = target - nums[i];
            if (map.containsKey(neededNumber)) {
                return new int[] { map.get(neededNumber), i };
            } else {
                map.put(nums[i], i);
            }
        }
        return new int[] { -1 };
    }

    public int[] twoSum2(int[] nums, int target) {
        for (int i = 0; i < nums.length; i++) {
            int neededNumber = target - nums[i];
            for (int j = i + 1; j < nums.length; j++) {
                if (nums[j] == neededNumber) {
                    return new int[] { i, j };
                }
            }
        }
        return new int[] { -1 };
    }
}
