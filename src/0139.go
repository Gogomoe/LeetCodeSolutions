package main

func wordBreak(s string, wordDict []string) bool {
	wordSet := map[string]bool{}
	wordMaxLen := 0
	for _, word := range wordDict {
		wordSet[word] = true
		wordMaxLen = max(wordMaxLen, len(word))
	}
	arr := make([]bool, len(s)+1)
	arr[0] = true
	for i := range len(s) + 1 {
		if !arr[i] {
			continue
		}
		for strLen := 1; strLen <= wordMaxLen; strLen++ {
			if i+strLen > len(s) {
				continue
			}
			str := s[i : i+strLen]
			if wordSet[str] {
				arr[i+strLen] = true
			}
		}
	}
	return arr[len(s)]
}
