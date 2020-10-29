// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn delete_duplicates(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut res_head = Some(Box::new(ListNode::new(0)));
        let mut prev_val: i32 = i32::min_value();
        let mut curr_node = &mut res_head;
        while let Some(node) = head.take() {
            if prev_val != node.val {
                if let Some(ref mut n) = curr_node {
                    n.next = Some(node.clone());
                    curr_node = &mut n.next;
                    prev_val = node.val;
                }
            }
            head = node.next;
        }
        if let Some(ref mut n) = curr_node {
            n.next = None;
        }
        res_head.unwrap().next
    }
}
