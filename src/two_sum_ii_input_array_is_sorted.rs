use std::collections::BTreeMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut diff_bt: BTreeMap<i32, i32> = BTreeMap::new();
        let mut result: Vec<i32> = Vec::new();

        for (index, i) in numbers.iter().enumerate() {
            let diff = target - i;
            if diff_bt.contains_key(&diff) {
                result.push(index as i32);
                result.push(*diff_bt.get(&diff).unwrap());
            } else {
                diff_bt.insert(*i, index as i32);
            }
        }
        result.sort();
        result
    }
}
