/*
 * [0973] k-closest-points-to-origin
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        let mut closest_k = Vec::new();
        for i in 0..k as usize {
            closest_k.push(points[i].clone());
        }
        closest_k
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![[-2, 2]]
        );
    }
}

// solution tests ends here
