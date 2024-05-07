use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        // Creating a HashMap to count occurrences of each value
        let mut m = HashMap::new();
        for coin in coins {
            *m.entry(coin).or_insert(0) += 1;
        }

        let mut ans = 0;

        // Iterate over sorted key-value pairs in the HashMap
        let mut pairs: Vec<_> = m.into_iter().collect();
        pairs.sort_by_key(|&(key, _)| key);

        for (key, count) in pairs {
            let x = key * count;
            if (ans + 1) >= key {
                ans += x;
            } else {
                break;
            }
        }

        ans + 1
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_maximum_consecutive() {
        let coins = vec![1, 3];
        let expected = 2; // Adjust the expected value according to your logic
        let result = Solution::get_maximum_consecutive(coins);
        assert_eq!(result, expected);
    }
}
