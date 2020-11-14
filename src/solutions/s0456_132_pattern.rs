// Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
// Return true if there is a 132 pattern in nums, otherwise, return false.
// Follow up: The O(n^2) is trivial, could you come up with the O(n logn) or the O(n) solution?
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: false
// Explanation: There is no 132 pattern in the sequence.
//
// Example 2:
//
// Input: nums = [3,1,4,2]
// Output: true
// Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
//
// Example 3:
//
// Input: nums = [-1,3,2,0]
// Output: true
// Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
//
//
// Constraints:
//
// 	n == nums.length
// 	1 <= n <= 10^4
// 	-10^9 <= nums[i] <= 10^9
//
// IMPLEMENTATION:
// https://leetcode.com/problems/132-pattern/discuss/94071/Single-pass-C%2B%2B-O(n)-space-and-time-solution-(8-lines)-with-detailed-explanation.

// 1) Each time we store a new number, we first pop out all numbers smaller than that number.
// 2) The most recently popped number is candidate for s3.
// 3) Once we encounter any number smaller than s3, we know we found a valid sequence since s1 < s3 implies s1 < s2

pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut s3 = std::i32::MIN;
        let mut stack = Vec::new();
        for &num in nums.iter().rev() {
            if num < s3 {
                return true;
            }
            while let Some(elem) = stack.last() {
                if num > *elem {
                    s3 = *elem;
                    let _ = stack.pop();
                } else {
                    break;
                }
            }
            stack.push(num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_456() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
        assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    }
}
