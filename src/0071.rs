use std::collections::VecDeque;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut names = Vec::new();
        let mut str: VecDeque<char> = path.chars().collect();
        while !str.is_empty() {
            remove_slash(&mut str);
            if str.is_empty() {
                break;
            }
            let next = next_part(&mut str);
            match next.as_ref() {
                ".." => { names.pop(); }
                "." => {}
                _ => { names.push(next); }
            }
        }
        return format!("/{}", names.join("/"));
    }
}

fn next_part(str: &mut VecDeque<char>) -> String {
    let mut result = String::new();
    while !str.is_empty() && *str.front().unwrap() != '/' {
        result.push(str.pop_front().unwrap());
    }
    return result;
}

fn remove_slash(str: &mut VecDeque<char>) {
    while !str.is_empty() && *str.front().unwrap() == '/' {
        str.pop_front();
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::simplify_path("/home/".to_string())); // "/home"
    println!("{}", Solution::simplify_path("/../".to_string())); // "/"
    println!("{}", Solution::simplify_path("/home//foo/".to_string())); // "/home/foo"
    println!("{}", Solution::simplify_path("/a/./b/../../c/".to_string())); // "/c"
}