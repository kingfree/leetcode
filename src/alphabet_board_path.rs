impl Solution {
    fn path(src: (i32, i32), dst: (i32, i32)) -> String {
        let mut res = String::new();
        let mut src = (src.0, src.1);
        if src.0 == 5 && dst.0 < 5 {
            res.push('U');
            src.0 -= 1;
        } // 例外：先上
        match dst.1 - src.1 {
            d if d < 0 => {
                for _ in d..0 {
                    res.push('L');
                }
            }
            d if d > 0 => {
                for _ in 0..d {
                    res.push('R');
                }
            }
            _ => {}
        } // 先左右
        match dst.0 - src.0 {
            d if d < 0 => {
                for _ in d..0 {
                    res.push('U');
                }
            }
            d if d > 0 => {
                for _ in 0..d {
                    res.push('D');
                }
            }
            _ => {}
        } // 再上下
        res.push('!');
        res
    }

    pub fn alphabet_board_path(target: String) -> String {
        // let board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"];
        let mut ans = String::new();
        let mut pos = (0, 0);
        let mut nxt = (0, 0);
        let a = 'a' as u8;
        for c in target.as_bytes() {
            nxt.0 = (c - a) as i32 / 5;
            nxt.1 = (c - a) as i32 % 5;
            ans.push_str(&Solution::path(pos, nxt));
            pos = nxt;
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::alphabet_board_path("leet".to_string()),
            "RDD!RRRUU!!DDD!".to_string()
        );
        assert_eq!(
            Solution::alphabet_board_path("code".to_string()),
            "RR!RRDD!LUU!R!".to_string()
        );
    }

}
