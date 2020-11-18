// Given two arrays A and B of equal size, the advantage of A with respect to B is the number of indices i for which A[i] > B[i].
//
// Return any permutation of A that maximizes its advantage with respect to B.
//
// <div>
// Example 1:
//
// Input: A = <span id="example-input-1-1">[2,7,11,15]</span>, B = <span id="example-input-1-2">[1,10,4,11]</span>
// Output: <span id="example-output-1">[2,11,7,15]</span>
//
// <div>
// Example 2:
//
// Input: A = <span id="example-input-2-1">[12,24,8,32]</span>, B = <span id="example-input-2-2">[13,25,32,11]</span>
// Output: <span id="example-output-2">[24,32,8,12]</span>
//
// Note:
//
// <ol>
// 	1 <= A.length = B.length <= 10000
// 	0 <= A[i] <= 10^9
// 	0 <= B[i] <= 10^9
// </ol>
// </div>
// </div>
//
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Node(usize, i32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution {}

impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; a.len()];

        a.sort();
        let mut heap = BinaryHeap::new();
        for (idx, &value) in b.iter().enumerate() {
            heap.push(Node(idx, value));
        }

        let (mut i, mut j) = (0, a.len() - 1);
        while let Some(Node(idx, val)) = heap.pop() {
            if val < a[j] {
                result[idx] = a[j];
                if j > 0 {
                    j -= 1;
                }
            } else {
                result[idx] = a[i];
                if i < a.len() - 1 {
                    i += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_870() {
        assert_eq!(
            Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
        assert_eq!(
            Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
            vec![24, 32, 8, 12]
        );
        assert_eq!(
            Solution::advantage_count(vec![1, 24, 56, 10], vec![3, 11, 12, 80]),
            vec![10, 24, 56, 1]
        );
    }
}
