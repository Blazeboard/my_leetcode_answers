/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 *
 * https://leetcode-cn.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (40.96%)
 * Likes:    6937
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 2.5M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
 * 
 * 请你将两个数相加，并以相同形式返回一个表示和的链表。
 * 
 * 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：l1 = [2,4,3], l2 = [5,6,4]
 * 输出：[7,0,8]
 * 解释：342 + 465 = 807.
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：l1 = [0], l2 = [0]
 * 输出：[0]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * 输出：[8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 每个链表中的节点数在范围 [1, 100] 内
 * 0 
 * 题目数据保证列表表示的数字不含前导零
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut t = (l1, l2, 0, 0); // (list1, list2, sum, carry)
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry) if list.val + carry >= 10 => {
                    (list.next, None, list.val + carry - 10, 1)
                }
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    (list.next, None, list.val + carry, 0)
                }
                (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10 => {
                    (l1.next, l2.next, l1.val + l2.val + carry - 10, 1)
                }
                (Some(l1), Some(l2), _, carry) => {
                    (l1.next, l2.next, l1.val + l2.val + carry, 0)
                }
            };

            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = & mut tail.as_mut().unwrap().next;
        }
        result
    }
}
// @lc code=end

