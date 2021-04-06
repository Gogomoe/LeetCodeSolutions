impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result = Vec::new();
        dfs(&mut result, &candidates, &mut Vec::new(), &candidates.len() - 1, target);
        result
    }
}

fn dfs(result: &mut Vec<Vec<i32>>, candidates: &Vec<i32>, current: &mut Vec<i32>, pos: usize, mut target: i32) {
    if target == 0 {
        result.push(current.clone());
        return;
    }
    if pos != 0 {
        dfs(result, candidates, current, pos - 1, target);
    }
    if target >= candidates[pos] {
        current.push(candidates[pos]);
        dfs(result, candidates, current, pos, target - candidates[pos]);
        current.pop();
    }
}

struct Solution {}

fn main() {}