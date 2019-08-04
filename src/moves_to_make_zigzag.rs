use std::cmp::min;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let calc = |i| {
            let left = if i > 0 && nums[i - 1] <= nums[i] {
                nums[i] - nums[i - 1] + 1
            } else {
                0
            };
            let right = if i + 1 < nums.len() && nums[i + 1] <= nums[i] - left {
                (nums[i] - left) - nums[i + 1] + 1
            } else {
                0
            };
            left + right
        };
        // 大小大
        let mut ans1 = 0;
        for i in (1..nums.len()).step_by(2) {
            ans1 += calc(i);
        }
        // 小大小大
        let mut ans2 = 0;
        for i in (0..nums.len()).step_by(2) {
            ans2 += calc(i);
        }
        min(ans1, ans2)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::moves_to_make_zigzag(vec![1, 2, 3]), 2);
        assert_eq!(Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
    }

}
