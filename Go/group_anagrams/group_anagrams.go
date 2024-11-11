package group_anagrams

func sortString(s string) string {
	runeArr := []rune(s)

	for i := 0; i < len(runeArr); i++ {
		for j := i + 1; j < len(runeArr); j++ {
			if runeArr[i] > runeArr[j] {
				runeArr[i], runeArr[j] = runeArr[j], runeArr[i]
			}
		}
	}

	return string(runeArr)
}

func groupAnagrams(strs []string) [][]string {
	hashMap := make(map[string][]string)

	for _, str := range strs {
		sortedString := sortString(str)
		if _, ok := hashMap[sortedString]; ok {
			hashMap[sortedString] = append(hashMap[sortedString], str)
		} else {
			hashMap[sortString(str)] = []string{str}
		}
	}

	result := [][]string{}
	for _, v := range hashMap {
		result = append(result, v)
	}

	return result
}
