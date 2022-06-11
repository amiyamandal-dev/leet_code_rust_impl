pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mid = s.len() /2;
        let mut last = s.len() -1;
        for i in 0..mid{
            let t = s[i];
            s[i] = s[last];
            s[last] = t;
            last -=1;
        }
    }
}   