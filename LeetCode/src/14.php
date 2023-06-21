<?php

class Solution
{
  function longestCommonPrefix($strs)
  {
    $prefix = $strs[0];
    for ($i = 1; $i < sizeof($strs); ++$i) {
      while (strpos($strs[$i], $prefix) !== 0) {
        $prefix = substr($prefix, 0, -1);
        if (strlen($prefix) === 0) {
          return "";
        }
      }
    }
    return $prefix;
  }
}

$test = new Solution();
echo $test->longestCommonPrefix(["flower", "flow", "flight"]); // TEST: "fl"
echo $test->longestCommonPrefix(["dog", "racecar", "car"]); // TEST: ""
