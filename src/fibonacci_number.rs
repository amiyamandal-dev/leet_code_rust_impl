pub struct Solution;

impl Solution {
    fn F(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        Solution::F(n - 1) + Solution::F(n - 2)
    }

    pub fn fib(n: i32) -> i32 {
        Solution::F(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::fib(3));
    }
}
