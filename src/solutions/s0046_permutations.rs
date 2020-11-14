// Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
//
// Example 1:
// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// Example 2:
// Input: nums = [0,1]
// Output: [[0,1],[1,0]]
// Example 3:
// Input: nums = [1]
// Output: [[1]]
//
// Constraints:
//
// 	1 <= nums.length <= 6
// 	-10 <= nums[i] <= 10
// 	All the integers of nums are unique.
//
use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    fn permute_helper<T: Clone>(
        result: &mut Vec<Vec<T>>,
        used: &mut Vec<T>,
        unused: &mut VecDeque<T>,
    ) {
        if unused.is_empty() {
            result.push(used.to_vec());
        } else {
            for _ in 0..unused.len() {
                used.push(unused.pop_front().unwrap());
                Solution::permute_helper(result, used, unused);
                unused.push_back(used.pop().unwrap());
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Solution::permute_helper(&mut result, &mut Vec::new(), &mut VecDeque::from(nums));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]).sort(),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
            .sort()
        );
    }
}
