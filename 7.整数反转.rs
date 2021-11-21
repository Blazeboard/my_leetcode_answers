/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 *
 * https://leetcode-cn.com/problems/reverse-integer/description/
 *
 * algorithms
 * Easy (35.20%)
 * Likes:    3184
 * Dislikes: 0
 * Total Accepted:    862.8K
 * Total Submissions: 2.5M
 * Testcase Example:  '123'
 *
 * 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
 * 
 * 如果反转后整数超过 32 位的有符号整数的范围 [−2^31,  2^31 − 1] ，就返回 0。
 * 假设环境不允许存储 64 位整数（有符号或无符号）。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：x = 123
 * 输出：321
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：x = -123
 * 输出：-321
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：x = 120
 * 输出：21
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：x = 0
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * -2^31 
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut temp = x;
        let mut s: i32 = 0;
        while temp != 0 {
            match s.checked_mul(10) {
                None => return 0,
                Some(tem) => match tem.checked_add(temp % 10) {
                    None => return 0,
                    Some(tem1) => s = tem1,
                }
            }
            temp /= 10;
        }
        s
    }
}
// @lc code=end

