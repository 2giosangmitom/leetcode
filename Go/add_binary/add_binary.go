package add_binary

func trimLeadingZeros(s *string) {
	k := 0
	for k < len(*s) && (*s)[k] == '0' {
		k++
	}

	if k == len(*s) {
		*s = "0"
	} else {
		*s = (*s)[k:]
	}
}

func addBinary(a string, b string) string {
	trimLeadingZeros(&a)
	trimLeadingZeros(&b)

	len_a := len(a)
	len_b := len(b)

	// Swap a and b if length of `a` smaller than length of `b`
	if len_a < len_b {
		a, b = b, a
		len_a, len_b = len_b, len_a
	}

	j := len_b - 1
	carry := 0

	for i := len_a - 1; i >= 0; i-- {
		bit1 := int(a[i] - '0')
		sum := bit1 + carry

		if j >= 0 {
			sum += int(b[j] - '0')
			j--
		}

		bit := rune(sum%2 + '0')
		carry = sum / 2
		runes := []rune(a)
		runes[i] = bit
		a = string(runes)
	}

	if carry > 0 {
		a = "1" + a
	}

	return a
}
