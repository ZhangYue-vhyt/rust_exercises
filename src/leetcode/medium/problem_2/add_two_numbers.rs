use super::list_node::ListNode;

// Too complex to test linkedlist problems.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut p, mut q, mut sum, mut carry) = (&l1, &l2, 0, 0);
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();

    loop {
        match (p, q) {
            (Some(node_p), Some(node_q)) => {
                sum = carry + node_p.val + node_q.val;
                p = &(node_p.next);
                q = &(node_q.next);
            }
            (Some(node_p), None) => {
                sum = carry + node_p.val;
                p = &(node_p.next);
            }
            (None, Some(node_q)) => {
                sum = carry + node_q.val;
                q = &(node_q.next);
            }
            _ => {
                if carry > 0 {
                    sum = carry;
                } else {
                    break;
                }
            }
        }
        carry = sum / 10;
        let node = current.unwrap();
        node.next = Some(Box::new(ListNode::new(sum % 10)));
        current = node.next.as_mut();
    }

    head.unwrap().next
}
