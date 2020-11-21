// Given a non-empty array of integers, return the k most frequent elements.
// Example 1:
//
// Input: nums = <span id="example-input-1-1">[1,1,1,2,2,3]</span>, k = <span id="example-input-1-2">2</span>
// Output: <span id="example-output-1">[1,2]</span>
//
// <div>
// Example 2:
//
// Input: nums = <span id="example-input-2-1">[1]</span>, k = <span id="example-input-2-2">1</span>
// Output: <span id="example-output-2">[1]</span>
// </div>
// Note:
//
// 	You may assume k is always valid, 1 &le; k &le; number of unique elements.
// 	Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
// 	It's guaranteed that the answer is unique, in other words the set of the top k frequent elements is unique.
// 	You can return the answer in any order.
//
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            *counter.entry(num).or_default() += 1;
        }
        let mut heap = BinaryHeap::with_capacity((k + 1) as usize);
        for (x, count) in counter {
            heap.push(Reverse((count, x)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.iter().map(|Reverse((_, x))| *x).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_347() {
        let answer = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert!(answer == vec![1, 2] || answer == vec![2, 1]);

        let answer = Solution::top_k_frequent(vec![1, 1, 1, 1, 1, 2, 2, 3, 3, 3, 3], 2);
        assert!(answer == vec![1, 3] || answer == vec![3, 1]);
    }
}
