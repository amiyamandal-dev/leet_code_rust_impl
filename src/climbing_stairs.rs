struct Solution {}

impl Solution {
    fn steps_conter(steps_moved: i32, target: i32, memo: &mut Vec<i32>) -> i32 {
        if steps_moved == target {
            return 1;
        } else if steps_moved > target {
            return 0;
        }
        if memo[steps_moved as usize] > 0 {
            return memo[steps_moved as usize];
        }
        let s1 = Solution::steps_conter(steps_moved + 1, target, memo);
        let s2 = Solution::steps_conter(steps_moved + 2, target, memo);
        memo[steps_moved as usize] = s1 + s2;
        return s1 + s2;
    }

    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let mut memo = vec![0; (n + 1) as usize];
        return Solution::steps_conter(0, n, &mut memo);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
