pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let splits = s.split_ascii_whitespace();
        let mut result:Vec<String> = Vec::new();
        for i in splits{
            let t = i.chars().rev().collect::<String>();
            result.push(t);
        }
        result.join(" ")
        
    }
}