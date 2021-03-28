use std::collections::HashMap;

fn dfs(result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, map: &mut HashMap<i32, i32>, keys: &Vec<i32>) {
    let mut have_element = false;
    for key in keys.iter() {
        if map[key] != 0 {
            have_element = true;
            *map.get_mut(key).unwrap() -= 1;
            current.push(*key);
            dfs(result, current, map, keys);
            current.pop();
            *map.get_mut(key).unwrap() += 1;
        }
    }
    if !have_element {
        result.push(current.clone());
    }
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut map = HashMap::new();
        for i in nums.iter() {
            let value = map.entry(*i).or_insert(0);
            *value += 1;
        }
        let keys = map.keys().cloned().collect();
        dfs(&mut result, &mut Vec::new(), &mut map, &keys);
        result
    }
}

struct Solution {}

fn main() {}