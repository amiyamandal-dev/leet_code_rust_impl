pub struct Solution();

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if target > nums[mid as usize] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super:: Solution;
    #[test]
    fn it_works() {
        let result = Solution::search_insert(vec![1,3,5,6], 5);
        assert_eq!(result, 2);
    }
}
