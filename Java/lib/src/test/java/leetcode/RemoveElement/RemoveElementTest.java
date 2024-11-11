package leetcode.RemoveElement;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;
import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class RemoveElementTest {
  protected Solution solution = new Solution();

  static Stream<Arguments> provideTestCases() {
    return Stream.of(
        Arguments.of("continuous occurrences", new int[] {3, 2, 2, 3}, 3, 2, new int[] {2, 2}),
        Arguments.of(
            "multiple occurrences",
            new int[] {0, 1, 2, 2, 3, 0, 4, 2},
            2,
            5,
            new int[] {0, 1, 3, 0, 4}),
        Arguments.of("no occurrences", new int[] {1, 2, 3, 4, 5}, 6, 5, new int[] {1, 2, 3, 4, 5}),
        Arguments.of("all occurrences", new int[] {2, 2, 2, 2}, 2, 0, new int[] {}),
        Arguments.of("mixed occurrences", new int[] {4, 5, 4, 6, 4, 7}, 4, 3, new int[] {5, 6, 7}));
  }

  @ParameterizedTest(name = "{0}")
  @MethodSource("provideTestCases")
  void testRemoveElement(String name, int[] nums, int val, int want, int[] wantNums) {
    int actual = solution.removeElement(nums, val);

    assertEquals(want, actual);

    int[] actualNums = Arrays.copyOf(nums, actual);
    assertArrayEquals(wantNums, actualNums);
  }
}
