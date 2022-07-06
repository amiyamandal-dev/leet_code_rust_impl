pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let sorted_arr = Solution::merge(nums1, nums2);
        let mid = sorted_arr.len() / 2;
        if sorted_arr.len() % 2 == 0 {
            return (sorted_arr[mid] + sorted_arr[mid - 1]) as f64 / 2.0;
        }
        return sorted_arr[mid] as f64;
    }
    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr: Vec<i32> = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] > nums2[j] {
                sorted_arr.push(nums2[j]);
                j += 1;
            } else {
                sorted_arr.push(nums1[i]);
                i += 1;
            }
        }
        if i <= (nums1.len() - 1) {
            for t_i in i..nums1.len() {
                sorted_arr.push(nums1[t_i]);
            }
        }

        if j <= (nums2.len() - 1) {
            for t_j in j..nums2.len() {
                sorted_arr.push(nums2[t_j]);
            }
        }

        sorted_arr
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
    }
}
