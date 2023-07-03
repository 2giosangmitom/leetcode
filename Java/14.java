class Solution {
  public String longestCommonPrefix(String[] strs) {
    String prefix = strs[0];
    for (var i = 1; i < strs.length; ++i) {
      while (strs[i].indexOf(prefix) != 0) {
        prefix = prefix.substring(0, prefix.length() - 1);
        if (prefix.isEmpty()) return "";
      }
    }
    return prefix;
  }

  public static void main(String[] args) {
    var test = new Solution();
    String[] strs = {"flower", "flow", "flight"}; // TEST: => "fl"
    System.out.println(test.longestCommonPrefix(strs));
  }
}
