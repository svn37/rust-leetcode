// The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
//
// Now your job is to find the total Hamming distance between all pairs of the given numbers.</p>
//
//
// Example:<br />
//
// Input: 4, 14, 2
//
// Output: 6
//
// Explanation: In binary representation, the 4 is 0100, 14 is 1110, and 2 is 0010 (just
// showing the four bits relevant in this case). So the answer will be:
// HammingDistance(4, 14) + HammingDistance(4, 2) + HammingDistance(14, 2) = 2 + 2 + 2 = 6.
//
//
//
// Note:<br>
// <ol>
// Elements of the given array are in the range of 0  to 10^9
// Length of the array will not exceed 10^4.
// </ol>
//
pub struct Solution {}

// https://leetcode.com/problems/total-hamming-distance/discuss/96226/Java-O(n)-time-O(1)-Space
// For each bit position 1-32 in a 32-bit integer, we count the number of integers in the array which have that bit set.
// Then, if there are n integers in the array and k of them have a particular bit set and (n-k) do not, then that bit contributes k*(n-k) hamming distance to the total.

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total_count = 0;
        let len = nums.len() as i32;
        for i in 0..32 {
            let mut bit_count = 0; // the total possibilities is k*(n-k)
            for num in &nums {
                bit_count += (num >> i) & 1;
            }
            total_count += bit_count * (len - bit_count);
        }
        total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_477() {
        assert_eq!(
            Solution::total_hamming_distance(vec![4, 14, 2, 5, 6, 7, 8, 9, 10, 123, 890]),
            190
        );
        assert_eq!(
            Solution::total_hamming_distance(vec![
                185, 88, 191, 50, 4, 165, 104, 93, 138, 125, 94, 117, 28, 114, 161, 33
            ]),
            484
        );
    }
}
