import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class AddTwoNumbersTests {
  @Test
  @DisplayName("case 1")
  void test1() {
    ListNode l1 = new ListNode(2, new ListNode(4, new ListNode(3, null)));
    ListNode l2 = new ListNode(5, new ListNode(6, new ListNode(4, null)));
    ListNode want = new ListNode(7, new ListNode(0, new ListNode(8, null)));
    ListNode result = AddTwoNumbers.addTwoNumbers(l1, l2);
    assertThat(result).usingRecursiveComparison().isEqualTo(want);
  }

  @Test
  @DisplayName("case 2")
  void test2() {
    ListNode l1 = new ListNode(0, null);
    ListNode l2 = new ListNode(0, null);
    ListNode want = new ListNode(0, null);
    ListNode result = AddTwoNumbers.addTwoNumbers(l1, l2);
    assertThat(result).usingRecursiveComparison().isEqualTo(want);
  }

  @Test
  @DisplayName("case 3")
  void test3() {
    ListNode l1 =
        new ListNode(
            9,
            new ListNode(
                9,
                new ListNode(
                    9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, null)))))));
    ListNode l2 = new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, null))));
    ListNode want =
        new ListNode(
            8,
            new ListNode(
                9,
                new ListNode(
                    9,
                    new ListNode(
                        9,
                        new ListNode(
                            0, new ListNode(0, new ListNode(0, new ListNode(1, null))))))));
    ListNode result = AddTwoNumbers.addTwoNumbers(l1, l2);
    assertThat(result).usingRecursiveComparison().isEqualTo(want);
  }
}
