package leetcode.LongestCommonPrefix;

class Solution {
  public String longestCommonPrefix(String[] strs) {
    String longestPrefix = strs.length == 0 ? "" : strs[0];

    for (int i = 1; i < strs.length; i++) {
      while (strs[i].indexOf(longestPrefix) != 0) {
        longestPrefix = longestPrefix.substring(0, longestPrefix.length() - 1);
        if (longestPrefix.isEmpty()) {
          return "";
        }
      }
    }

    return longestPrefix;
  }
}
