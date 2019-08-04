use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut set = HashMap::new();
        let (mut i, mut j, mut ans) = (0, 0, 0);
        while i < n && j < n {
            if !set.contains_key(&s[j]) {
                set.insert(s[j], 1);
                // println!("insert {} => {:?} [{:?}]", s[j], set, s);
                j += 1;
                ans = cmp::max(ans, j - i);
            } else {
                set.remove(&s[i]);
                // println!("remove {} => {:?} [{:?}]", s[i], set, s);
                i += 1;
            }
        }
        ans as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

}
