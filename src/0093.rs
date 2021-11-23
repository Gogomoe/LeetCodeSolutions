impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        dfs(&s.chars().collect(), 0, &mut Vec::new(), &mut result);
        return result;
    }
}

fn dfs(chars: &Vec<char>, pos: usize, list: &mut Vec<usize>, result: &mut Vec<String>) {
    if list.len() == 3 {
        if valid(chars, pos, chars.len() - 1) {
            let mut str = String::new();
            let mut j = 0;
            for i in 0..chars.len() {
                str.push(chars[i]);
                if j < 3 && i == list[j] {
                    str.push('.');
                    j += 1;
                }
            }
            result.push(str);
        }
        return;
    }

    for i in 0..=2 {
        if valid(chars, pos, pos + i) {
            list.push(pos + i);
            dfs(chars, pos + i + 1, list, result);
            list.pop();
        }
    }
}

fn valid(chars: &Vec<char>, p1: usize, p2: usize) -> bool {
    if p1 > p2 || p2 >= chars.len() {
        return false;
    }
    return match p2 - p1 {
        0 => true,
        1 => chars[p1] != '0',
        2 => match chars[p1] {
            '0' => false,
            '1' => true,
            '2' => match chars[p1 + 1] {
                '0' | '1' | '2' | '3' | '4' => true,
                '5' => chars[p2] as u8 <= '5' as u8,
                _ => false
            }
            _ => false
        }
        _ => false
    };
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::restore_ip_addresses("25525511135".to_string())); // ["255.255.11.135","255.255.111.35"]
    println!("{:?}", Solution::restore_ip_addresses("0000".to_string())); // ["0.0.0.0"]
    println!("{:?}", Solution::restore_ip_addresses("1111".to_string())); // ["1.1.1.1"]
    println!("{:?}", Solution::restore_ip_addresses("010010".to_string())); // ["0.10.0.10","0.100.1.0"]
    println!("{:?}", Solution::restore_ip_addresses("101023".to_string())); // ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
}
