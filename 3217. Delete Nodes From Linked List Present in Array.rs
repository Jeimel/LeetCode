use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut set: HashSet<_> = nums.into_iter().collect();

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut current = &mut dummy;

        while let Some(next) = current.next.take() {
            // Skip current node, as the value is included in `nums`
            if set.contains(&next.val) {
                current.next = next.next;

                continue;
            }

            // Add node, which is not included in `nums` to our result
            current.next = Some(next);
            // Go to the next node 
            current = current.next.as_mut().unwrap();
        }

        dummy.next 
    }
}
