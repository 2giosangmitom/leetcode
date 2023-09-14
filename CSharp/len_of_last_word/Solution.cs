namespace length_of_last_word;

public class Solution {
    public static int LengthOfLastWord(string s) {
        int result = 0;
        s = s.Trim();
        for (int i = s.Length - 1; i >= 0 && s[i] != ' '; --i) {
            result++;
        }
        return result;
    }
}
