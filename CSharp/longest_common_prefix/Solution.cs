namespace longest_common_prefix {
    public class Solution {
        public static string LongestCommonPrefix(string[] strs) {
            string prefix = strs[0];
            foreach (var s in strs[1..]) {
                while (s.IndexOf(prefix) != 0) {
                    prefix = prefix[0..(prefix.Length - 1)];
                    if (prefix.Length == 0) return "";
                }
            }
            return prefix;
        }
    }
}