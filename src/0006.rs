impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let interval = (num_rows as usize - 1) * 2;
        let mut result = String::new();
        for i in 0..(num_rows as usize) {
            let mut pos = i;
            while pos < s.len() {
                result.push(chars[pos]);
                if i == 0 || i == num_rows as usize - 1 {
                    pos += interval
                } else if pos % interval < num_rows as usize {
                    pos += interval - 2 * i
                } else {
                    pos += 2 * i
                }
            }
        }

        result
    }
}

struct Solution {}

fn main() {}