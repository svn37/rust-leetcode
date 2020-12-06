// Given a sorted positive integer array nums and an integer n, add/patch elements to the array such that any number in range [1, n] inclusive can be formed by the sum of some elements in the array. Return the minimum number of patches required.
//
// Example 1:
//
//
// Input: nums = [1,3], n = 6
// Output: 1
// Explanation:
// Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
// Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
// Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
// So we only need 1 patch.
//
// Example 2:
//
//
// Input: nums = [1,5,10], n = 20
// Output: 2
// Explanation: The two patches can be [2, 4].
//
//
// Example 3:
//
//
// Input: nums = [1,2,2], n = 5
// Output: 0
//
// https://leetcode.com/problems/patching-array/discuss/78488/Solution-%2B-explanation
// Brilliant solution by StefanPochmann

pub struct Solution {}

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        // Convert to i64 to handle i32::MAX case
        let nums: Vec<_> = nums.iter().map(|&num| num as i64).collect();
        let n = n as i64;

        let (mut miss, mut added, mut i) = (1, 0, 0);
        while miss <= n {
            if i < nums.len() && nums[i] <= miss {
                miss += nums[i];
                i += 1;
            } else {
                miss += miss;
                added += 1;
            }
        }
        added
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_330() {
        let answer = Solution::min_patches(vec![1, 3], 1_000_000);
        assert!(answer == 19);

        let answer = Solution::min_patches(vec![2, 4, 6, 10, 56, 57, 1000], 10_001);
        assert!(answer == 8);

        let answer = Solution::min_patches(vec![1, 2, 31, 33], i32::MAX);
        assert!(answer == 28);
    }
}
