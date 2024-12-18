#ifndef REMOVE_DUPLICATES_FROM_SORTED_LIST
#define REMOVE_DUPLICATES_FROM_SORTED_LIST

#include <linked_list.hpp>

class Solution {
public:
  ListNode *deleteDuplicates(ListNode *head) {
    if (head == nullptr)
      return nullptr;

    ListNode *dummyHead = head;
    for (ListNode *tail = head->next; tail != nullptr; tail = tail->next) {
      if (dummyHead->val != tail->val) {
        dummyHead->next = tail;
        dummyHead = dummyHead->next;
      }
    }
    dummyHead->next = nullptr;

    return head;
  }
};

#endif // !REMOVE_DUPLICATES_FROM_SORTED_LIST
