package validparentheses

func isValid(s string) bool {
	if len(s)%2 != 0 {
		return false
	}

	stack := []rune{}

	for _, char := range s {
		if char == '(' || char == '[' || char == '{' {
			stack = append(stack, char)
		} else {
			if len(stack) == 0 {
				return false
			} else if (char == ')' && stack[len(stack)-1] == '(') || (char == ']' && stack[len(stack)-1] == '[') || (char == '}' && stack[len(stack)-1] == '{') {
				stack = stack[:len(stack)-1]
			} else {
				return false
			}
		}
	}

	return len(stack) == 0
}
