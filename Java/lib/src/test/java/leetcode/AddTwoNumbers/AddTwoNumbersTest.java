package leetcode.AddTwoNumbers;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import java.util.stream.Stream;
import leetcode.Helpers.ListNode;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

public class AddTwoNumbersTest {
  protected Solution solution = new Solution();

  static Stream<Arguments> testsProvider() {
    return Stream.of(
        Arguments.of(
            "same number of digits",
            ListNode.fromArray(new int[] {2, 4, 3}),
            ListNode.fromArray(new int[] {5, 6, 4}),
            new int[] {7, 0, 8}),
        Arguments.of(
            "different number of digits",
            ListNode.fromArray(new int[] {2, 4, 3}),
            ListNode.fromArray(new int[] {5, 6}),
            new int[] {7, 0, 4}),
        Arguments.of(
            "one list is empty",
            ListNode.fromArray(new int[] {}),
            ListNode.fromArray(new int[] {5, 6}),
            new int[] {5, 6}),
        Arguments.of(
            "both lists are empty",
            ListNode.fromArray(new int[] {}),
            ListNode.fromArray(new int[] {}),
            new int[] {}));
  }

  @ParameterizedTest(name = "{0}")
  @MethodSource("testsProvider")
  void test(String name, ListNode l1, ListNode l2, int[] expected) {
    ListNode result = solution.addTwoNumbers(l1, l2);
    assertArrayEquals(expected, result != null ? result.toArray() : new int[] {});
  }
}
