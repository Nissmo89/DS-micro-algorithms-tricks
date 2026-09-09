struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    /**
     * Reverses a singly linked list recursively.
     *
     * The base case for the recursion is when the list is empty or has only one node,
     * in which case the list is already reversed (or trivially reversed).
     * Otherwise, the function recursively reverses the rest of the list (from head->next onwards).
     * Once the rest of the list is reversed, the original head node needs to be appended
     * to the end of the reversed sublist.
     *
     * @param head The head node of the singly linked list.
     * @return The head node of the reversed linked list.
     */
    ListNode* reverseListRecursive(ListNode* head) {
        // Base case: If the list is empty or has only one node, return it.
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        // Recursively reverse the rest of the list.
        // `reversedTail` will be the new head of the reversed sublist.
        ListNode* reversedTail = reverseListRecursive(head->next);

        // Now, `head->next` still points to the original second node.
        // Let the original second node be `node2`. After recursion, `node2` is the tail
        // of the reversed sublist, and its `next` pointer is nullptr.
        // We want to make `node2->next` point back to `head`.
        // `head->next` is `node2`.
        head->next->next = head;

        // Set the original head's `next` pointer to nullptr, as it will be the new tail.
        head->next = nullptr;

        // `reversedTail` is the new head of the fully reversed list.
        return reversedTail;
    }
};
