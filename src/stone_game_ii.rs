use std::cmp::max;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut s = {
            let mut vec = Vec::with_capacity(n);
            vec.resize(n, 0);
            vec
        };
        let mut f = {
            let mut arr = Vec::with_capacity(n);
            arr.resize(n, s.clone());
            arr
        };
        s[n - 1] = piles[n - 1];
        for i in (0..n - 1).rev() {
            s[i] = s[i + 1] + piles[i];
        }
        for i in (0..n).rev() {
            for j in 0..n {
                let m = j + 1;
                if m * 2 >= n - i {
                    f[i][j] = s[i];
                } else {
                    for k in 1..2 * m + 1 {
                        f[i][j] = max(f[i][j], s[i] - f[i + k][max(k, m) - 1]);
                    }
                }
            }
        }
        f[0][0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }

}
