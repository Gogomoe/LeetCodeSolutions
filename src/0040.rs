impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result = Vec::new();
        let mut list = Vec::new();
        dfs(0, 0, &mut list, &mut result, target, &candidates);
        return result;
    }
}

fn dfs(pos: usize, sum: i32, list: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, target: i32, candidates: &Vec<i32>) {
    if sum == target {
        result.push(list.clone());
        return;
    }
    if pos == candidates.len() {
        return;
    }

    let mut i = pos;
    while i < candidates.len() {
        let next = find_next_distinct(i, candidates);

        let value = candidates[i];
        let mut new_sum = sum;
        for _ in i..next {
            new_sum += candidates[i];
            if new_sum > target {
                break;
            }
            list.push(value);
            dfs(next, new_sum, list, result, target, candidates);
        }
        while !list.is_empty() && *list.last().unwrap() == value {
            list.pop();
        }

        i = next;
    }
}

fn find_next_distinct(mut i: usize, arr: &Vec<i32>) -> usize {
    let value = arr[i];
    while i < arr.len() && arr[i] == value {
        i += 1;
    }
    return i;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)); // [[1, 2, 5], [1, 7], [1, 1, 6], [2, 6]]
    println!("{:?}", Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)); // [[1, 2, 2], [5]]
}