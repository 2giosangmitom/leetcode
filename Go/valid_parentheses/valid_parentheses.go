package valid_parentheses

func isValid(s string) bool {
	length := len(s)
	if length%2 != 0 {
		return false
	}

	stack := []rune{}
	matchingBrackets := map[rune]rune{
		')': '(',
		'}': '{',
		']': '[',
	}

	for _, char := range s {
		if char == '(' || char == '{' || char == '[' {
			stack = append(stack, char)
		} else {
			if len(stack) == 0 || stack[len(stack)-1] != matchingBrackets[char] {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}

	return len(stack) == 0
}
