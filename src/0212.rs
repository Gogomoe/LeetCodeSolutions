use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter;

struct TireTree {
    word: Option<String>,
    nodes: HashMap<char, TireTree>,
}

impl TireTree {
    pub fn new() -> Self {
        return TireTree { word: None, nodes: HashMap::new() };
    }

    pub fn insert(&mut self, str: String) {
        let chars: Vec<char> = str.chars().collect();
        let mut node = self;
        for char in chars {
            node = node.nodes.entry(char).or_insert(TireTree::new());
        }
        node.word = Some(str);
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut tire = TireTree::new();
        for word in words {
            tire.insert(word);
        }

        let mut result = HashSet::new();
        let m = board.len();
        let n = board[0].len();
        let mut visit: Vec<Vec<bool>> = iter::repeat(
            iter::repeat(false).take(n).collect()
        ).take(m).collect();
        for i in 0..m {
            for j in 0..n {
                dfs(i, j, &board, &tire, &mut visit, &mut result);
            }
        }
        return result.into_iter().collect();
    }
}

fn dfs(y: usize, x: usize, board: &Vec<Vec<char>>, parent: &TireTree, visit: &mut Vec<Vec<bool>>, result: &mut HashSet<String>) {
    if visit[y][x] { return; }
    let tire = match parent.nodes.borrow().get(&board[y][x]) {
        Some(it) => it,
        None => return
    };

    visit[y][x] = true;
    if let Some(word) = &tire.word {
        result.insert(word.clone());
    }
    if x > 0 {
        dfs(y, x - 1, board, tire, visit, result);
    }
    if y > 0 {
        dfs(y - 1, x, board, tire, visit, result);
    }
    if x < board[0].len() - 1 {
        dfs(y, x + 1, board, tire, visit, result);
    }
    if y < board.len() - 1 {
        dfs(y + 1, x, board, tire, visit, result);
    }

    visit[y][x] = false;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::find_words(
        vec![vec!['o', 'a', 'a', 'n'], vec!['e', 't', 'a', 'e'], vec!['i', 'h', 'k', 'r'], vec!['i', 'f', 'l', 'v']],
        vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()],
    )); // ["eat","oath"]
    println!("{:?}", Solution::find_words(
        vec![vec!['a', 'b'], vec!['c', 'd']],
        vec!["abcb".to_string()],
    )); // []
    println!("{:?}", Solution::find_words(
        vec![vec!['a']],
        vec!["a".to_string()],
    )); // ["a"]
}