impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n < 0 {
            if n == i32::MIN {
                n = -(n / 2);
                x = (1.0 / x) * (1.0 / x);
            } else {
                n = -n;
                x = 1.0 / x;
            }
        }

        if n == 1 {
            return x;
        }

        return if n % 2 == 0 {
            Solution::my_pow(x * x, n / 2)
        } else {
            x * Solution::my_pow(x * x, n / 2)
        };
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::my_pow(2.0, 10)); // 1024
    println!("{}", Solution::my_pow(2.1, 3)); // 9.261
    println!("{}", Solution::my_pow(2.0, -2)); // 0.25
    println!("{}", Solution::my_pow(2.00000, -2147483648)); // 0.25
}