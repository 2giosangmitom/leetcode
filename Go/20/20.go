package valid_parentheses

func isValid(s string) bool {
	length := len(s)
	if length%2 != 0 {
		return false
	}

	stack := []rune{}
	for i := 0; i < length; i++ {
		if (s[i] == '(') || s[i] == '{' || s[i] == '[' {
			stack = append(stack, rune(s[i]))
		} else {
			if len(stack) == 0 {
				return false
			}

			if s[i] == ')' && stack[len(stack)-1] == '(' || s[i] == '}' && stack[len(stack)-1] == '{' || s[i] == ']' && stack[len(stack)-1] == '[' {
				stack = stack[:len(stack)-1]
			} else {
				return false
			}
		}
	}

	return len(stack) == 0
}
