# frozen_string_literal: true

require 'add_two_numbers'

# @param {ListNode} l1
# @param {ListNode} l2
# @return {Boolean}
def are_equal(l1, l2)
  while !l1.nil? && !l2.nil?
    return false if l1.val != l2.val

    l1 = l1.next if !l1.nil?
    l2 = l2.next if !l2.nil?
  end
  return true
end

describe 'add_two_numbers' do
  cases = [
    {
      l1: ListNode.new(2, ListNode.new(4, ListNode.new(3, nil))),
      l2: ListNode.new(5, ListNode.new(6, ListNode.new(4, nil))),
      want: ListNode.new(7, ListNode.new(0, ListNode.new(8, nil)))
    },
    {
      l1: ListNode.new(0, nil),
      l2: ListNode.new(0, nil),
      want: ListNode.new(0, nil),
    },
    # rubocop:disable Layout/LineLength
    {
      l1: ListNode.new(9, ListNode.new(9, ListNode.new(9, ListNode.new(9, ListNode.new(9, ListNode.new(9, ListNode.new(9, nil))))))),
      l2: ListNode.new(9, ListNode.new(9, ListNode.new(9, ListNode.new(9, nil)))),
      want: ListNode.new(8, ListNode.new(9, ListNode.new(9, ListNode.new(9, ListNode.new(0, ListNode.new(0, ListNode.new(0, ListNode.new(1, nil)))))))),
    }
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result = add_two_numbers(c[:l1], c[:l2])
      expect(are_equal(result, c[:want])).to be true
    end
  end
end
