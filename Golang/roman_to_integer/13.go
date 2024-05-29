package romantointeger

func romanToInt(s string) int {
	romanMap := map[string]int{
		"I": 1,
		"V": 5,
		"X": 10,
		"L": 50,
		"C": 100,
		"D": 500,
		"M": 1000,
	}

	result := 0

	// Reverse `s`
	func(x *string) {
		r := ""
		for _, v := range *x {
			r = string(v) + r
		}
		*x = r
	}(&s)

	for _, c := range s {
		number, ok := romanMap[string(c)]
		if !ok {
			return -1
		}

		if number*4 < result {
			result -= number
		} else {
			result += number
		}
	}

	return result
}
