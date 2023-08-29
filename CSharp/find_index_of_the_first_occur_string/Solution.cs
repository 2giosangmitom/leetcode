namespace find_index_of_the_first_occur_string {
    public class Solution {
        public static int StrStr(string haystack, string needle) {
            int haystackLength = haystack.Length;
            int needleLength = needle.Length;
            if (needleLength > haystackLength) {
                return -1;
            }
            for (int i = 0; i <= haystackLength - needleLength; ++i) {
                if (haystack[i..(i + needleLength)] == needle) {
                    return i;
                }
            }
            return -1;
        }
    }
}