class ListNode {
    int val;
    ListNode next;
    ListNode(int x) { val = x; next = null; }
}

public class LinkedListCycleDetector {

    /**
     * Detects if a singly linked list has a cycle using Floyd's Tortoise and Hare algorithm.
     *
     * This algorithm uses two pointers, a 'slow' pointer that moves one step at a time,
     * and a 'fast' pointer that moves two steps at a time. If there is a cycle,
     * the fast pointer will eventually catch up to the slow pointer.
     *
     * @param head The head of the linked list.
     * @return true if a cycle exists, false otherwise.
     */
    public boolean hasCycle(ListNode head) {
        ListNode slow = head;
        ListNode fast = head;

        // The loop continues as long as fast and fast.next are not null.
        // If fast or fast.next is null, it means we've reached the end of the list
        // without the pointers meeting, hence no cycle.
        while (fast != null && fast.next != null) {
            slow = slow.next;          // Move slow pointer by one step
            fast = fast.next.next;      // Move fast pointer by two steps

            // If slow and fast pointers meet, a cycle is detected.
            if (slow == fast) {
                return true;
            }
        }

        // If the loop finishes without pointers meeting, there is no cycle.
        return false;
    }
}