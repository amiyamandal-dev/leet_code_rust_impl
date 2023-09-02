pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut t: Vec<i32> = vec![];
        for i in left..right + 1 {
            let temp = Solution::to_binary(i);
            if Solution::is_prime(temp) == true {
                t.push(1);
            }
        }
        t.iter().sum()
    }

    fn to_binary(val: i32) -> i32 {
        let mut cnt: i32 = 31;
        let mut tmp: i32 = 0;
        let mut binary_vector: Vec<i32> = vec![];
        while cnt >= 0 {
            tmp = val & (1 << cnt);
            if tmp > 0 {
                binary_vector.push(1);
            } else {
                binary_vector.push(0)
            }
            cnt = cnt - 1;
        }
        let all_sum_val: i32 = binary_vector.iter().sum();
        all_sum_val
    }

    fn is_prime(val: i32) -> bool {
        if val == 1 {
            return false;
        }
        let mut i = 2;
        while i * i <= val {
            if val % i == 0 {
                return false;
            }
            i += 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::count_prime_set_bits(10, 15));
    }
}
