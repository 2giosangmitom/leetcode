<?php

// Valid Parentheses
class Solution
{
  function isValid($s)
  {
    if (strlen($s) % 2 !== 0) {
      return false;
    }
    $stack = [];
    for ($i = 0; $i < strlen($s); ++$i) {
      if ($s[$i] === "(" or $s[$i] === "{" or $s[$i] === "[") {
        array_push($stack, $s[$i]);
      } else {
        if ($s[$i] === ")" and end($stack) === "(") {
          array_pop($stack);
        } else if ($s[$i] === "}" and end($stack) === "{") {
          array_pop($stack);
        } else if ($s[$i] === "]" and end($stack) === "[") {
          array_pop($stack);
        } else return false;
      }
    }
    return sizeof($stack) === 0;
  }
}

$test = new Solution();
var_dump($test->isValid("({})[]")); // TEST: bool(true)
var_dump($test->isValid("(){}[]")); // TEST: bool(true)
var_dump($test->isValid("({)}")); // TEST: bool(false)
