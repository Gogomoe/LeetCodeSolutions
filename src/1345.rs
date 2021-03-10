use std::collections::{HashMap, VecDeque};
use std::cmp::min;
use std::iter;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut steps: Vec<i32> = iter::repeat(-1).take(n).collect();
        let mut nums = HashMap::new();

        for i in 0..n {
            let it = arr[i];
            nums.entry(it).or_insert(Vec::new()).push(i);
        }

        let mut queue = VecDeque::new();
        queue.push_back(0);
        steps[0] = 0;

        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            let it = arr[i];
            let step = steps[i];

            if i != 0 && steps[i - 1] == -1 {
                steps[i - 1] = step + 1;
                queue.push_back(i - 1);
            }
            if i != n - 1 && steps[i + 1] == -1 {
                steps[i + 1] = step + 1;
                queue.push_back(i + 1);
            }
            for same in nums.get(&it).unwrap().iter() {
                if steps[*same] == -1 {
                    steps[*same] = step + 1;
                    queue.push_back(*same);
                }
            }
            nums.get_mut(&it).unwrap().clear();
        }

        return steps[n - 1];
    }
}

struct Solution {}

fn main() {}