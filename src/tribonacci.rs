impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut t = Vec::with_capacity(40);
        t.push(0);
        t.push(1);
        t.push(1);
        for i in 3..n + 1 {
            t.push(t[i - 1] + t[i - 2] + t[i - 3])
        }
        // println!("{:?}", t);
        t[n]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }

}
