impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        Solution::dfs(String::new(), 0, n, &mut result);
        return result;
    }

    fn dfs(str: String, stack: i32, left: i32, result: &mut Vec<String>) {
        if left == 0 && stack == 0 {
            result.push(str);
            return;
        }
        if left != 0 {
            Solution::dfs(str.clone() + "(", stack + 1, left - 1, result);
        }
        if stack != 0 {
            Solution::dfs(str.clone() + ")", stack - 1, left, result);
        }
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3)); // ["((()))","(()())","(())()","()(())","()()()"]
}