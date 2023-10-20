import 'package:test/test.dart';
import 'package:leetcode/add_two_numbers.dart';

typedef TestList = List<({ListNode l1, ListNode l2, ListNode? want})>;

bool areEqual(ListNode? l1, ListNode? l2) {
  while (l1 != null && l2 != null) {
    if (l1.val != l2.val) {
      return false;
    }
    l1 = l1.next;
    l2 = l2.next;
  }
  return true;
}

void main() {
  TestList cases = [
    (
      l1: ListNode(2, ListNode(4, ListNode(3, null))),
      l2: ListNode(5, ListNode(6, ListNode(4, null))),
      want: ListNode(7, ListNode(0, ListNode(8, null)))
    ),
    (
      l1: ListNode(0, null),
      l2: ListNode(0, null),
      want: ListNode(0, null),
    ),
    (
      l1: ListNode(
        9,
        ListNode(
          9,
          ListNode(
            9,
            ListNode(
              9,
              ListNode(9, ListNode(9, ListNode(9, null))),
            ),
          ),
        ),
      ),
      l2: ListNode(
        9,
        ListNode(9, ListNode(9, ListNode(9, null))),
      ),
      want: ListNode(
        8,
        ListNode(
          9,
          ListNode(
            9,
            ListNode(
              9,
              ListNode(
                0,
                ListNode(0, ListNode(0, ListNode(1, null))),
              ),
            ),
          ),
        ),
      ),
    ),
  ];

  for (var tt in cases) {
    test('add two numbers', () {
      ListNode? result = Solution().addTwoNumbers(tt.l1, tt.l2);
      expect(areEqual(result, tt.want), true);
    });
  }
}
