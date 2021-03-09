use std::cmp::{min, max};

fn go_right(diff: &mut Vec<i32>, cost: &mut i32, r: &mut usize) {
    while *r != diff.len() {
        let reduce = min(*cost, diff[*r]);
        diff[*r] -= reduce;
        *cost -= reduce;
        if diff[*r] == 0 {
            *r += 1;
        } else {
            break;
        }
    }
}

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let mut diff = Vec::new();
        let len = s.len();
        let s = s.into_bytes();
        let t = t.into_bytes();

        for i in 0..len {
            diff.push((s[i] as i32 - t[i] as i32).abs())
        }

        let mut cost = max_cost;
        let mut l = 0;
        let mut r = 0;

        go_right(&mut diff, &mut cost, &mut r);

        let mut result = r - l;

        while r != len {
            let origin = (s[l] as i32 - t[l] as i32).abs();
            l += 1;
            cost += origin;

            go_right(&mut diff, &mut cost, &mut r);

            result = max(result, r - l);
        }

        result as i32
    }
}

struct Solution {}

fn main() {
    println!("{}",
             Solution::equal_substring(
                 String::from("abcd"),
                 String::from("bcdf"),
                 3,
             )
    );
    println!("{}",
             Solution::equal_substring(
                 String::from("abcd"),
                 String::from("acde"),
                 0,
             )
    );
}