impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m, mut s) = (grid.len(), grid[0].len(), 0);
        let mut l = {
            let mut vec = Vec::with_capacity(m + 1);
            vec.resize(m + 1, 0);
            let mut arr = Vec::with_capacity(n + 1);
            arr.resize(n + 1, vec);
            arr
        };
        let mut u = l.clone();
        for i in 0..n {
            for j in 0..m {
                s += grid[i][j];
                l[i][j + 1] = l[i][j] + grid[i][j]; // 左边
                u[i + 1][j] = u[i][j] + grid[i][j]; // 上边
            }
        }
        // println!("{:?}\n{:?}\n{}", l, u, s);
        if s <= 0 {
            return 0;
        }
        let mut edge = (0, 0, 0, 0);
        for e in (0..std::cmp::min(n, m) + 1).rev() {
            for i in 0..n - e + 1 {
                for j in 0..m - e + 1 {
                    edge.0 = l[i][j + e] - l[i][j];
                    edge.1 = l[i + e - 1][j + e] - l[i + e - 1][j];
                    edge.2 = u[i + e][j] - u[i][j];
                    edge.3 = u[i + e][j + e - 1] - u[i][j + e - 1];
                    // println!("e = {} {:?}", e, edge);
                    if e as i32 == edge.0
                        && edge.0 == edge.1
                        && edge.1 == edge.2
                        && edge.2 == edge.3
                    {
                        return (e * e) as i32;
                    }
                }
            }
        }
        0
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            9
        );
        assert_eq!(
            Solution::largest1_bordered_square(vec![vec![1, 1, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::largest1_bordered_square(vec![vec![0, 1], vec![1, 0]]),
            1
        );
    }

}
