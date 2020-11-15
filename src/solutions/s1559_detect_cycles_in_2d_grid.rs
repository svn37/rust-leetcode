// Given a 2D array of characters grid of size m x n, you need to find if there exists any cycle consisting of the same value in grid.
// A cycle is a path of length 4 or more in the grid that starts and ends at the same cell. From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), if it has the same value of the current cell.
// Also, you cannot move to the cell that you visited in your last move. For example, the cycle (1, 1) -> (1, 2) -> (1, 1) is invalid because from (1, 2) we visited (1, 1) which was the last visited cell.
// Return true if any cycle of the same value exists in grid, otherwise, return false.
//
// Example 1:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/1.png" style="width: 231px; height: 152px;" />
//
// Input: grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
// Output: true
// Explanation: There are two valid cycles shown in different colors in the image below:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/11.png" style="width: 225px; height: 163px;" />
//
// Example 2:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/22.png" style="width: 236px; height: 154px;" />
//
// Input: grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
// Output: true
// Explanation: There is only one valid cycle highlighted in the image below:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/2.png" style="width: 229px; height: 157px;" />
//
// Example 3:
// <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/3.png" style="width: 183px; height: 120px;" />
//
// Input: grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
// Output: false
//
//
// Constraints:
//
// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m <= 500
// 	1 <= n <= 500
// 	grid consists only of lowercase English letters.
//
use std::usize;

pub struct Solution {}

impl Solution {
    pub fn contains_cycle(mut grid: Vec<Vec<char>>) -> bool {
        if grid.is_empty() {
            return false;
        }
        let (height, width) = (grid.len(), grid[0].len());
        if height < 1 || width < 1 {
            return false;
        }
        for i in 0..height {
            for j in 0..width {
                if grid[i][j].is_lowercase()
                    && Solution::find_cycle(
                        i,
                        j,
                        grid[i][j],
                        (usize::MAX, usize::MAX),
                        &mut grid,
                        height,
                        width,
                    )
                {
                    return true;
                }
            }
        }
        false
    }

    fn find_cycle(
        x: usize,
        y: usize,
        letter: char,
        prev: (usize, usize),
        grid: &mut Vec<Vec<char>>,
        height: usize,
        width: usize,
    ) -> bool {
        let upcase_letter = letter.to_uppercase().next().unwrap();
        if grid[x][y] == upcase_letter {
            return true;
        }
        if grid[x][y] != letter {
            return false;
        }
        grid[x][y] = upcase_letter;

        return (x > 0
            && prev != (x - 1, y)
            && Solution::find_cycle(x - 1, y, letter, (x, y), grid, height, width))
            || (x + 1 < height
                && prev != (x + 1, y)
                && Solution::find_cycle(x + 1, y, letter, (x, y), grid, height, width))
            || (y > 0
                && prev != (x, y - 1)
                && Solution::find_cycle(x, y - 1, letter, (x, y), grid, height, width))
            || (y + 1 < width
                && prev != (x, y + 1)
                && Solution::find_cycle(x, y + 1, letter, (x, y), grid, height, width));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1559() {
        assert_eq!(
            Solution::contains_cycle(vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['a', 'b', 'b', 'a'],
                vec!['a', 'b', 'b', 'a'],
                vec!['a', 'a', 'a', 'a']
            ]),
            true
        );
        assert_eq!(
            Solution::contains_cycle(vec![
                vec!['a', 'a', 'c', 'a'],
                vec!['a', 'b', 'b', 'a'],
                vec!['a', 'b', 'c', 'a'],
                vec!['a', 'a', 'a', 'a']
            ]),
            false
        );
        assert_eq!(
            Solution::contains_cycle(vec![
                vec!['c', 'c', 'c', 'a'],
                vec!['c', 'd', 'c', 'c'],
                vec!['c', 'c', 'e', 'c'],
                vec!['f', 'c', 'c', 'c']
            ]),
            true
        );
        assert_eq!(
            Solution::contains_cycle(vec![
                vec!['a', 'b', 'b'],
                vec!['b', 'z', 'b'],
                vec!['b', 'b', 'a'],
            ]),
            false
        );
        assert_eq!(Solution::contains_cycle(vec![vec!['a'],]), false);
    }
}
