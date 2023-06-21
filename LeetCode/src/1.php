<?php

class Solution
{
    public function twoSum($nums, $target)
    {
        $hashMap = [];
        foreach ($nums as $key => $value) {
            $needNumber = $target - $value;
            if (isset($hashMap[$needNumber])) {
                return [$hashMap[$needNumber], $key];
            }
            $hashMap[$value] = $key;
        }
        return -1;
    }
}

$test = new Solution();
$arr = [2, 7, 11, 15];
print_r($test->twoSum($arr, 9)); // TEST: [0, 1]
$arr2 = [3, 2, 4];
print_r($test->twoSum($arr2, 6)); // TEST: [1, 2]
$arr3 = [2, 4, 6, 12, 9, 3];
print_r($test->twoSum($arr3, 18)); // TEST: [2, 3]
