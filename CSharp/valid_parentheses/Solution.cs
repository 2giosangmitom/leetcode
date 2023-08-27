namespace valid_parenthese {
    public class Solution {
        public static bool IsValid(string s) {
            if (s.Length % 2 != 0) return false;
            var stack = new Stack<char>();
            for (int i = 0; i < s.Length; i++) {
                if (s[i] == '{' || s[i] == '[' || s[i] == '(') {
                    stack.Push(s[i]);
                } else {
                    if (stack.Count == 0) {
                        return false;
                    }
                    if (s[i].Equals('}') && stack.Peek().Equals('{')) {
                        stack.Pop();
                    } else if (s[i].Equals(']') && stack.Peek().Equals('[')) {
                        stack.Pop();
                    } else if (s[i].Equals(')') && stack.Peek().Equals('(')) {
                        stack.Pop();
                    } else return false;
                }
            }
            return stack.Count == 0;
        }
    }
}
