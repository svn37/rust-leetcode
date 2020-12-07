//
// We are given N different types of stickers.  Each sticker has a lowercase English word on it.
//
// You would like to spell out the given target string by cutting individual letters from your collection of stickers and rearranging them.
//
// You can use each sticker more than once if you want, and you have infinite quantities of each sticker.
//
// What is the minimum number of stickers that you need to spell out the target?  If the task is impossible, return -1.
//
//
// Example 1:
// Input:
// ["with", "example", "science"], "thehat"
//
//
// Output:
// 3
//
//
// Explanation:
// We can use 2 "with" stickers, and 1 "example" sticker.
// After cutting and rearrange the letters of those stickers, we can form the target "thehat".
// Also, this is the minimum number of stickers necessary to form the target string.
//
//
// Example 2:
// Input:
// ["notice", "possible"], "basicbasic"
//
//
// Output:
// -1
//
//
// Explanation:
// We can't form the target "basicbasic" from cutting letters from the given stickers.
//
//
// Note:
// stickers has length in the range [1, 50].
// stickers consists of lowercase English words (without apostrophes).
// target has length in the range [1, 15], and consists of lowercase English letters.
// In all test cases, all words were chosen <u>randomly</u> from the 1000 most common US English words, and the target was chosen as a concatenation of two random words.
// The time limit may be more challenging than usual.  It is expected that a 50 sticker test case can be solved within 35ms on average.
//

// https://leetcode.com/problems/stickers-to-spell-word/discuss/108318/C%2B%2BJavaPython-DP-%2B-Memoization-with-optimization-29-ms-(C%2B%2B)
// Brilliant solution by zestypanda.
use std::collections::HashMap;
use std::{char, cmp};

pub struct Solution {}

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut mp = vec![vec![0; 26]; stickers.len()];
        for i in 0..mp.len() {
            for c in stickers[i].chars() {
                mp[i][c as usize - 'a' as usize] += 1;
            }
        }
        let mut dp = HashMap::new();
        dp.insert(String::new(), 0);

        fn helper(dp: &mut HashMap<String, i32>, mp: &[Vec<usize>], target: String) -> i32 {
            if let Some(&res) = dp.get(&target) {
                return res;
            }

            let mut tar = vec![0; 26];
            for c in target.chars() {
                tar[c as usize - 'a' as usize] += 1;
            }
            let mut ans = std::i32::MAX;
            for i in 0..mp.len() {
                let first_char = target.chars().next().unwrap();
                if mp[i][first_char as usize - 'a' as usize] == 0 {
                    continue;
                }
                let mut s = String::new();
                for (j, _) in tar.iter().enumerate() {
                    if tar[j] > mp[i][j] {
                        let c = char::from_u32((j + 'a' as usize) as u32).unwrap();
                        (0..tar[j] - mp[i][j]).for_each(|_| {
                            s.push(c);
                        });
                    }
                }
                let tmp = helper(dp, mp, s);
                if tmp == -1 {
                    continue;
                }
                ans = cmp::min(ans, 1 + tmp);
            }
            let res = if ans == std::i32::MAX { -1 } else { ans };
            dp.insert(target, res);
            res
        }
        helper(&mut dp, &mp, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_691() {
        assert_eq!(
            Solution::min_stickers(
                vec![
                    "with".to_owned(),
                    "example".to_owned(),
                    "science".to_owned()
                ],
                "thehat".to_owned()
            ),
            3
        );
        assert_eq!(
            Solution::min_stickers(
                vec!["notice".to_owned(), "possible".to_owned(),],
                "basicbasic".to_owned()
            ),
            -1
        );
    }
}
