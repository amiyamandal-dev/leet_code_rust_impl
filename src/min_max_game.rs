use std::cmp;

pub struct Solution;

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut new_nums = nums[0];
        if nums.len() == 1 {
            return new_nums;
        } else {
            let mut t: Vec<i32> = nums;

            while t.len() > 1 {
                let mut temp: Vec<i32> = vec![];
                let mut index_val: usize = 0;
                while index_val < t.len() / 2 {
                    if index_val % 2 == 0 {
                        temp.push(cmp::min(t[2 * index_val], t[2 * index_val + 1]));
                    } else {
                        temp.push(cmp::max(t[2 * index_val], t[2 * index_val + 1]));
                    }
                    index_val = index_val + 1;
                }
                print!("{:?}\n", temp);
                t = temp;
            }
            t[0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(22, Solution::min_max_game(vec![70, 38, 21, 22]));
        assert_eq!(1, Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]));
    }
    #[test]
    fn it_works_2() {
        assert_eq!(
            850,
            Solution::min_max_game(vec![999, 939, 892, 175, 114, 464, 850, 107])
        );
    }
}
