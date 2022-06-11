pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let last_index = nums.len() - 1;
        return Solution::binary_search(nums, target, 0, last_index);
    }

    fn binary_search(nums: Vec<i32>, target: i32, start_index: usize, last_index: usize) -> i32 {
        if start_index < last_index {
            let mid = (start_index + last_index) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else {
                if nums[mid] < target {
                    return Solution::binary_search(nums, target, mid + 1, last_index);
                } else {
                    if mid == 0 {
                        return -1;
                    }
                    return Solution::binary_search(nums, target, start_index, mid - 1);
                }
            }
        } else if start_index == last_index {
            if nums[start_index] == target {
                return start_index as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(-1, Solution::search(vec![5,], -5));
        assert_eq!(-1, Solution::search(vec![2, 5], 0));
    }
}
