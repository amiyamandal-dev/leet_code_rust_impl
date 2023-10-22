pub struct Solution;

fn backtrack(current: &mut Vec<i32>, remaining: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if remaining.is_empty() {
        result.push(current.clone());
        return;
    }
    
    let len = remaining.len();
    for _ in 0..len {
        let n = remaining.remove(0);
        current.push(n);
        backtrack(current, remaining, result);
        current.pop();
        remaining.push(n);
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut nums = nums;
        backtrack(&mut current, &mut nums, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        print!("{:?}", Solution::permute(vec![1, 2, 3]));
    }
}
