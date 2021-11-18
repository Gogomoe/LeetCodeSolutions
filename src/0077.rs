impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut list = Vec::new();
        let mut result = Vec::new();
        dfs(1, &mut list, n, k, &mut result);
        return result;
    }
}

fn dfs(i: i32, list: &mut Vec<i32>, n: i32, k: i32, result: &mut Vec<Vec<i32>>) {
    if k == 0 {
        result.push(list.clone());
        return;
    }
    for j in i..=(n + 1 - k) {
        list.push(j);
        dfs(j + 1, list, n, k - 1, result);
        list.pop();
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::combine(4, 2));
    // [
    //   [2,4],
    //   [3,4],
    //   [2,3],
    //   [1,2],
    //   [1,3],
    //   [1,4],
    // ]
    println!("{:?}", Solution::combine(1, 1));
    // [[1]]
}