pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for ch in s.chars() {
            match ch {
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '(' => stack.push(')'),
                '}' | ']' | ')' => {
                    let t = stack.pop();
                    if Some(ch) != t {
                        return false;
                    }
                }
                _ => (),
            }
        }
        return stack.is_empty();
    }
}
