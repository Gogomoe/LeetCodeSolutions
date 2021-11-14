impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let x = m + n - 2;
        let y = m - 1;
        // C(x,y)
        let mut result = 1u64;
        for i in 1..=y {
            result *= (x - y + i) as u64;
            result /= i as u64;
        }
        return result as i32;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::unique_paths(3, 7)); // 28
    println!("{}", Solution::unique_paths(3, 2)); // 3
}
