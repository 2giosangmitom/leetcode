package validparentheses

func isValid(s string) bool {
	stack := []string{}
	for _, char := range s {
		if char == '{' || char == '(' || char == '[' {
			stack = append(stack, string(char))
		} else {
			if len(stack) == 0 {
				return false
			} else if char == '}' && stack[len(stack)-1] == "{" {
				stack = stack[:len(stack)-1]
			} else if char == ')' && stack[len(stack)-1] == "(" {
				stack = stack[:len(stack)-1]
			} else if char == ']' && stack[len(stack)-1] == "[" {
				stack = stack[:len(stack)-1]
			} else {
				return false
			}
		}
	}
	return len(stack) == 0
}
