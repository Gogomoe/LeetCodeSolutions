package main

import "fmt"

func isValid(s string) bool {
	mapping := map[rune]rune{
		')': '(',
		']': '[',
		'}': '{',
	}
	stack := []rune{}

	for _, ch := range s {
		switch ch {
		case '(', '[', '{':
			stack = append(stack, ch)
		case ')', ']', '}':
			if len(stack) == 0 {
				return false
			}
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if top != mapping[ch] {
				return false
			}
		}
	}
	return len(stack) == 0
}

func main() {
	fmt.Println(isValid("()[]{}"))
	fmt.Println(isValid("(]"))
}
