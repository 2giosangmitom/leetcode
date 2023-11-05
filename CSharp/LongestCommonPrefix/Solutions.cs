namespace CSharp.LongestCommonPrefix;

public class Solution {
    public static string LongestCommonPrefix(string[] strs) {
        string prefix = strs.First();

        for (int i = 1; i < strs.Length; i++) {
            while (strs[i].IndexOf(prefix, StringComparison.Ordinal) != 0) {
                prefix = prefix[..(prefix.Length - 1)];
                if (prefix.Length == 0) {
                    return "";
                }
            }
        }

        return prefix;
    }
}