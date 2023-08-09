package main

func romanToInt(s string) int {
	reverseString := func(str *string) {
		result := ""
		for _, char := range *str {
			result = string(char) + result
		}
		*str = result
	}
	reverseString(&s)

	result := 0
	var number int

	for _, char := range s {
		switch char {
		case 'I':
			number = 1
		case 'V':
			number = 5
		case 'X':
			number = 10
		case 'L':
			number = 50
		case 'C':
			number = 100
		case 'D':
			number = 500
		case 'M':
			number = 1000
		default:
			panic("Invalid roman number")
		}

		if number*4 < result {
			result -= number
		} else {
			result += number
		}
	}
	return result
}
