package leetcode.TwoSum;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class TwoSumTest {
  protected Solution solution = new Solution();

  static Stream<Arguments> provideTestCases() {
    return Stream.of(
        Arguments.of(new int[] {2, 7, 11, 15}, 9, new int[] {0, 1}, "increasing order"),
        Arguments.of(new int[] {3, 2, 4}, 6, new int[] {1, 2}, "different order"),
        Arguments.of(new int[] {3, 3}, 6, new int[] {0, 1}, "same numbers"),
        Arguments.of(new int[] {}, 5, new int[] {}, "empty array"),
        Arguments.of(new int[] {1, 2, 3}, 10, new int[] {}, "no solution"));
  }

  @ParameterizedTest(name = "{2}")
  @MethodSource("provideTestCases")
  void testTwoSum(int[] nums, int target, int[] expected, String displayName) {
    int[] result = solution.twoSum(nums, target);
    assertArrayEquals(expected, result);
  }
}
