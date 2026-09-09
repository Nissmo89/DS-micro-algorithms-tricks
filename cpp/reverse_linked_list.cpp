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
     * Reverses a singly linked list iteratively.
     *
     * This function takes the head of a singly linked list and reverses it in-place.
     * It uses three pointers: `prev`, `current`, and `nextNode`.
     * `prev` starts as nullptr, `current` starts as the head.
     * In each step, `current`'s `next` pointer is redirected to `prev`.
     * Then, `prev` is updated to `current`, and `current` is updated to `nextNode`.
     * The process continues until `current` becomes nullptr, at which point `prev` points to the new head.
     *
     * @param head The head node of the singly linked list.
     * @return The head node of the reversed linked list.
     */
    ListNode* reverseList(ListNode* head) {
        ListNode* prev = nullptr;
        ListNode* current = head;

        while (current != nullptr) {
            ListNode* nextNode = current->next; // Store the next node
            current->next = prev;               // Reverse the current node's pointer
            prev = current;                     // Move prev one step forward
            current = nextNode;                 // Move current one step forward
        }

        return prev; // prev is the new head of the reversed list
    }
};
