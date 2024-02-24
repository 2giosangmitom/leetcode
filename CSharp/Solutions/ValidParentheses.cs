namespace Solutions;

public class ValidParentheses {
    public static bool IsValid(string s) {
        if (s.Length % 2 != 0) {
            return false;
        }

        Stack<char> stack = [];
        for (int i = 0; i < s.Length; i++) {
            if (s[i] is '{' or '[' or '(') {
                stack.Push(s[i]);
            } else {
                if (stack.Count == 0) {
                    return false;
                }
                if (s[i] == '}' && stack.Peek() == '{') {
                    _ = stack.Pop();
                } else if (s[i] == ']' && stack.Peek() == '[') {
                    _ = stack.Pop();
                } else if (s[i] == ')' && stack.Peek() == '(') {
                    _ = stack.Pop();
                } else {
                    return false;
                }
            }
        }

        return stack.Count == 0;
    }
}