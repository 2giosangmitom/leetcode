package leetcode.TwoSum;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class TwoSumTest {
  Solution solution = new Solution();

  static Stream<Arguments> provideTestCases() {
    return Stream.of(
        Arguments.of(new int[] {2, 7, 11, 15}, 9, new int[] {0, 1}, "Case 1: standard case"),
        Arguments.of(new int[] {3, 2, 4}, 6, new int[] {1, 2}, "Case 2: normal case"),
        Arguments.of(new int[] {3, 3}, 6, new int[] {0, 1}, "Case 3: same numbers"));
  }

  @ParameterizedTest(name = "{3}")
  @MethodSource("provideTestCases")
  void testTwoSum(int[] nums, int target, int[] expected, String displayName) {
    int[] result = solution.twoSum(nums, target);
    assertArrayEquals(expected, result);
  }
}
