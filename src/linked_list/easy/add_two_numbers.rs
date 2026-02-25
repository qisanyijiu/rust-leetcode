#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut digits = Vec::new();
        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            digits.push(sum % 10);
            carry = sum / 10;
        }
        if carry > 0 {
            digits.push(carry);
        }
        let mut head = None;
        for &val in digits.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = ListNode::new(2);
        let l2 = ListNode::new(4);
        let result = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(result, Some(Box::new(ListNode::new(6))));
    }
}