package main

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}
	pre2 := 1
	pre := 2
	res := 0
	for i := 3; i <= n; i++ {
		res = pre + pre2
		pre2 = pre
		pre = res
	}
	return res
}

func main() {
	println(climbStairs(5))
}
