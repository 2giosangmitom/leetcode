# frozen_string_literal: true

# Definition for singly-linked list.
class ListNode
  attr_accessor :val, :next

  def initialize(val = 0, _next = nil)
    @val = val
    @next = _next
  end
end

# @param {ListNode} l1
# @param {ListNode} l2
# @return {ListNode}
def add_two_numbers(l1, l2)
  dummy_head = ListNode.new
  tail = dummy_head
  carry = 0

  while l1 != nil || l2 != nil || carry != 0
    digit1 = l1 == nil ? 0 : l1.val
    digit2 = l2 == nil ? 0 : l2.val

    sum = digit1 + digit2 + carry
    digit = sum % 10
    carry = (sum / 10).floor

    new_node = ListNode.new(digit)
    tail.next = new_node
    tail = tail.next

    l1 = l1.next if !l1.nil?
    l2 = l2.next if !l2.nil?
  end

  return dummy_head.next
end
