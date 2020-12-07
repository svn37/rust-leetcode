// You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
//
// Define a pair (u,v) which consists of one element from the first array and one element from the second array.
//
// Find the k pairs (u1,v1),(u2,v2) ...(uk,vk) with the smallest sums.
//
// Example 1:
//
//
// Input: nums1 = <span id="example-input-1-1">[1,7,11]</span>, nums2 = <span id="example-input-1-2">[2,4,6]</span>, k = <span id="example-input-1-3">3</span>
// Output: <span id="example-output-1">[[1,2],[1,4],[1,6]]
// Explanation: </span>The first 3 pairs are returned from the sequence:
//              [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
//
// Example 2:
//
//
// Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
// Output: [1,1],[1,1]<span>
// Explanation: </span>The first 2 pairs are returned from the sequence:
//              [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
//
// Example 3:
//
//
// Input: nums1 = [1,2], nums2 = [3], k = 3
// Output: [1,3],[2,3]<span>
// Explanation: </span>All possible pairs are returned from the sequence: [1,3],[2,3]
//
//
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct SumReverse(i32, i32, usize);

impl Ord for SumReverse {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.0 + other.1).cmp(&(self.0 + self.1))
    }
}

impl PartialOrd for SumReverse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((other.0 + other.1).cmp(&(self.0 + self.1)))
    }
}

pub struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums1.is_empty() || nums2.is_empty() {
            return result;
        }
        let mut heap = BinaryHeap::new();
        for i in 0..k {
            let i = i as usize;
            if i == nums1.len() {
                break;
            }
            heap.push(SumReverse(nums1[i], nums2[0], 0));
        }
        while let Some(SumReverse(num1, num2, i)) = heap.pop() {
            result.push(vec![num1, num2]);
            if result.len() == k as usize {
                return result;
            }
            if i + 1 == nums2.len() {
                continue;
            }
            heap.push(SumReverse(num1, nums2[i + 1], i + 1));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_373() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2, 3, 8, 9], vec![2, 4, 5, 7], 10),
            vec![
                vec![1, 2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 4],
                vec![1, 4],
                vec![3, 2],
                vec![2, 4],
                vec![1, 5],
                vec![1, 5],
                vec![3, 4]
            ]
        );
        assert_eq!(
            Solution::k_smallest_pairs(
                vec![16, 19, 53, 65, 67, 68],
                vec![11, 21, 26, 52, 56, 78],
                5
            ),
            vec![
                vec![16, 11],
                vec![19, 11],
                vec![16, 21],
                vec![19, 21],
                vec![16, 26],
            ]
        );
    }
}
