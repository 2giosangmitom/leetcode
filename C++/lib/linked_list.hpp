#ifndef LINKED_LIST_HPP
#define LINKED_LIST_HPP

#include <cstddef>
#include <memory>
#include <vector>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;

  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}

  static unique_ptr<ListNode> from(const vector<int> &values) {
    if (values.empty()) {
      return nullptr;
    }

    auto head = make_unique<ListNode>(values[0]);
    ListNode *tail = head.get();

    for (size_t i = 1; i < values.size(); ++i) {
      tail->next = new ListNode(values[i]);
      tail = tail->next;
    }

    return head;
  }

  bool operator==(ListNode *other) {
    ListNode *current = this;
    while (current != nullptr && other != nullptr) {
      if (current->val != other->val) {
        return false;
      }
      current = current->next;
      other = other->next;
    }
    return current == nullptr && other == nullptr;
  }
};

#endif // LINKED_LIST_HPP
