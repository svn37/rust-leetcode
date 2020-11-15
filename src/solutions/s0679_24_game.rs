//
// You have 4 cards each containing a number from 1 to 9.  You need to judge whether they could operated through *, /, +, -, (, ) to get the value of 24.
//
//
// Example 1:<br />
//
// Input: [4, 1, 8, 7]
// Output: True
// Explanation: (8-4) * (7-1) = 24
//
//
//
// Example 2:<br />
//
// Input: [1, 2, 1, 2]
// Output: False
//
//
//
// Note:<br>
// <ol>
// The division operator / represents real division, not integer division.  For example, 4 / (1 - 2/3) = 12.
// Every operation done is between two numbers.  In particular, we cannot use - as a unary operator.  For example, with [1, 1, 1, 1] as input, the expression -1 - 1 - 1 - 1 is not allowed.
// You cannot concatenate numbers together.  For example, if the input is [1, 2, 1, 2], we cannot write this as 12 + 12.
// </ol>
//
//
pub struct Solution {}

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        let nums: Vec<_> = nums.iter().map(|num| *num as f32).collect();
        Solution::permute(nums)
    }

    fn permute(nums: Vec<f32>) -> bool {
        if nums.len() == 1 && format!("{:.1$}", nums[0], 3) == "24.000" {
            return true;
        }
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if Solution::permute(Solution::apply(&nums, i, j, &Solution::add))
                    || Solution::permute(Solution::apply(&nums, i, j, &Solution::sub))
                    || Solution::permute(Solution::apply(&nums, i, j, &Solution::subr))
                    || Solution::permute(Solution::apply(&nums, i, j, &Solution::mul))
                    || Solution::permute(Solution::apply(&nums, i, j, &Solution::div))
                    || Solution::permute(Solution::apply(&nums, i, j, &Solution::divr))
                {
                    return true;
                }
            }
        }
        false
    }

    fn apply(nums: &Vec<f32>, i: usize, j: usize, func: &dyn Fn(f32, f32) -> f32) -> Vec<f32> {
        let mut copy = nums.clone();
        copy[i] = func(nums[i], nums[j]);
        copy.remove(j);
        copy
    }

    fn add(a: f32, b: f32) -> f32 {
        return a + b;
    }

    fn sub(a: f32, b: f32) -> f32 {
        return a - b;
    }

    fn subr(a: f32, b: f32) -> f32 {
        return b - a;
    }

    fn mul(a: f32, b: f32) -> f32 {
        return a * b;
    }

    fn div(a: f32, b: f32) -> f32 {
        return a / b;
    }

    fn divr(a: f32, b: f32) -> f32 {
        return b / a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_679() {
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
        assert_eq!(Solution::judge_point24(vec![4, 5, 8, 7]), true);
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
        assert_eq!(Solution::judge_point24(vec![24, 13, 2, 10]), true);
        assert_eq!(Solution::judge_point24(vec![116, 7, 2, 19]), false);
    }
}
