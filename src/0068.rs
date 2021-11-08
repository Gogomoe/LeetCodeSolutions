impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut line = Vec::new();
        let mut width = 0;
        for word in words {
            let new_width = width + word.len() + if width == 0 { 0 } else { 1 };
            if new_width > max_width {
                result.push(Solution::make_line(&line, width, max_width));
                line.clear();
                width = 0;
            }
            width += word.len() + if width == 0 { 0 } else { 1 };
            line.push(word);
        }
        let mut str = line.join(" ") as String;
        while str.len() != max_width {
            str.push(' ');
        }
        result.push(str);
        return result;
    }

    fn make_line(line: &Vec<String>, width: usize, max_width: usize) -> String {
        if line.len() == 1 {
            let mut str = line.first().unwrap().clone();
            while str.len() != max_width {
                str.push(' ');
            }
            return str;
        }
        let remain_space = max_width - width;
        let each = remain_space / (line.len() - 1);
        let first = remain_space % (line.len() - 1);

        let mut result = String::new();
        result.push_str(line.first().unwrap());
        for i in 1..line.len() {
            result.push(' ');
            for _ in 0..each {
                result.push(' ');
            }
            if i <= first {
                result.push(' ');
            }
            result.push_str(line.get(i).unwrap());
        }
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::full_justify(vec![
        "This".to_string(), "is".to_string(), "an".to_string(), "example".to_string(),
        "of".to_string(), "text".to_string(), "justification.".to_string(),
    ], 16)); // ["This    is    an", "example  of text", "justification.  "]
    println!("{:?}", Solution::full_justify(vec![
        "What".to_string(), "must".to_string(), "be".to_string(), "acknowledgment".to_string(),
        "shall".to_string(), "be".to_string(),
    ], 16)); // ["What   must   be", "acknowledgment  ", "shall be        "]
    println!("{:?}", Solution::full_justify(vec![
        "Science".to_string(), "is".to_string(), "what".to_string(), "we".to_string(),
        "understand".to_string(), "well".to_string(), "enough".to_string(), "to".to_string(),
        "explain".to_string(), "to".to_string(), "a".to_string(), "computer.".to_string(),
        "Art".to_string(), "is".to_string(), "everything".to_string(), "else".to_string(),
        "we".to_string(), "do".to_string(),
    ], 20)); // ["Science  is  what we", "understand      well", "enough to explain to", "a  computer.  Art is", "everything  else  we", "do                  "]
}