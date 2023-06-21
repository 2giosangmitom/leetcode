<?php

// Remove Element
class Solution
{
  function removeElement(&$nums, $val)
  {
    $nums = array_filter($nums, fn ($item) => $item !== $val);
    return sizeof($nums);
  }
}

$test = new Solution();
$nums = [3, 2, 2, 3];
echo $test->removeElement($nums, 3); // TEST: 2
echo "\n";
$nums = [0, 1, 2, 2, 3, 0, 4, 2];
echo $test->removeElement($nums, 2); // TEST: 5
