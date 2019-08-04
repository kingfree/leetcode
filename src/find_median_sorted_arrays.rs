use std::cmp;

impl Solution {
    fn get_k_th(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        // println!("{:?}\n{:?}\n{}", nums1, nums2, k);
        let n1 = nums1.len();
        let n2 = nums2.len();
        if n1 > n2 {
            return Solution::get_k_th(&nums2[..], &nums1[..], k);
        } else if n1 == 0 {
            return nums2[k - 1];
        } else if k == 1 {
            return cmp::min(nums1[0], nums2[0]);
        }

        let i = cmp::min(n1, k / 2) - 1;
        let j = cmp::min(n2, k / 2) - 1;
        if nums1[i] > nums2[j] {
            Solution::get_k_th(&nums1[..], &nums2[j + 1..], k - (j + 1))
        } else {
            Solution::get_k_th(&nums1[i + 1..], &nums2[..], k - (i + 1))
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k1 = (n1 + n2 + 1) / 2;
        let k2 = (n1 + n2 + 2) / 2;
        let ans = Solution::get_k_th(&nums1[..], &nums2[..], k1);
        if k1 == k2 {
            ans as f64
        } else {
            (ans + Solution::get_k_th(&nums1[..], &nums2[..], k2)) as f64 * 0.5
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_median_sorted_arrays(
                vec![1, 3, 4, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            ),
            4.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 4, 9], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            4.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

}
