impl Solution {
    pub fn longest_palindrome(str: String) -> String {
        let s: Vec<char> = str.chars().collect();
        let len = str.len();

        if len == 0 {
            return str;
        }

        let mut result_i = 0;
        let mut result_len = 0;
        let mut arr = vec![vec![0 as usize; len]; len];
        for l in 0..len {
            for i in 0..(len - l) {
                if l <= 1 && s[i] == s[i + l] {
                    arr[l][i] = l + 1
                } else if l >= 2 && s[i] == s[i + l] && arr[l - 2][i + 1] != 0 {
                    arr[l][i] = l + 1
                }
                if arr[l][i] > result_len {
                    result_len = arr[l][i];
                    result_i = i;
                }
            }
        }

        str[result_i..(result_i + result_len)].to_string()
    }
}


struct Solution {}

fn main() {
    println!("{}", Solution::longest_palindrome("a".to_string()));
    println!("{}", Solution::longest_palindrome("babad".to_string()));
    println!("{}", Solution::longest_palindrome("aacabdkacaa".to_string()));
}