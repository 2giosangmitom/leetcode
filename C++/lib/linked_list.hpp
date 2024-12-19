#ifndef LINKED_LIST_HPP
#define LINKED_LIST_HPP

#include <cstddef>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

/**
 * A simple singly linked list node structure.
 */
class ListNode {
public:
  int val;
  ListNode *next;

  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}

  /**
   * Constructs a linked list from a vector of integers.
   * If the vector is empty, returns nullptr.
   *
   * @param values A vector of integers to construct the list.
   * @return A pointer to the head of the linked list.
   */
  static ListNode *from(const std::vector<int> &values) {
    if (values.empty()) {
      return nullptr;
    }

    ListNode *head = new ListNode(values[0]);
    ListNode *tail = head;

    for (size_t i = 1; i < values.size(); ++i) {
      tail->next = new ListNode(values[i]);
      tail = tail->next;
    }

    return head;
  }

  /**
   * Converts the linked list to a string representation.
   *
   * @return A string representation of the list in the format "1 -> 2 -> 3".
   */
  string to_string() {
    stringstream ss;
    const ListNode *current = this;
    if (!current) {
      return "";
    }

    while (current) {
      ss << current->val;
      if (current->next) {
        ss << " -> ";
      }
      current = current->next;
    }

    return ss.str();
  }

  /**
   * Deletes the linked list starting from this node.
   */
  ~ListNode() {
    ListNode *current = this;
    while (current) {
      ListNode *nextNode = current->next;
      delete current;
      current = nextNode;
    }
  }
};

#endif // LINKED_LIST_HPP
