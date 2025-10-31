impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut carry = 0;

        // We iterate until both lists are empty and our carry is zero.
        // The idea is to implement written addition
        while l1.is_some() || l2.is_some() || carry != 0 {
            // Get the respective node value, if the node is empty just take zero
            let val1 = l1.as_ref().map_or(0, |node| node.val);
            let val2 = l2.as_ref().map_or(0, |node| node.val);

            // Calculate sum and carry
            let sum = val1 + val2 + carry;
            carry = sum / 10;

            // Add the next digit to our result
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            // Go to the next element in our list
            current = current.next.as_mut().unwrap();

            // Update the pointers of both lists
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        // Dummy is empty, so we have to take the next element
        dummy.next
    }
}
