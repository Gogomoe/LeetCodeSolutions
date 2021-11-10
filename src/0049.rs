use std::collections::HashMap;
use std::num::Wrapping;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<u64, Vec<String>> = HashMap::new();
        for str in strs {
            result.entry(hash(&str)).or_insert(Vec::new()).push(str);
        }
        return result.values().cloned().collect();
    }
}

fn hash(str: &String) -> u64 {
    // should not use prime 2. because enough 2's will make it overflow to zero
    let table = vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 107, 109];
    let mut hash = Wrapping(1u64);
    for c in str.chars() {
        hash *= Wrapping(table[(c as u8 - 'a' as u8) as usize]);
    }
    return hash.0;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::group_anagrams(vec![
        "eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(),
        "nat".to_string(), "bat".to_string(),
    ])); // [["bat"],["nat","tan"],["ate","eat","tea"]]
    println!("{:?}", Solution::group_anagrams(vec![
        "".to_string()
    ])); // [[""]]
    println!("{:?}", Solution::group_anagrams(vec![
        "a".to_string()
    ])); // [["a"]]
    println!("{:?}", Solution::group_anagrams(vec![
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
    ]));
    // [
    //      ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //      ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"]
    // ]
}