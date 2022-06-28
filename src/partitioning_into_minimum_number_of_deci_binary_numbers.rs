pub struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let char_vec: Vec<char> = n.chars().collect();
        let mut max_val = std::i32::MIN;
        for i in char_vec.iter() {
            let t = (i.to_string()).parse::<i32>().unwrap();
            if max_val < t {
                max_val = t;
            }
        }
        max_val
    }
}
