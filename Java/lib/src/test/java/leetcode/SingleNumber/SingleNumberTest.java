package leetcode.SingleNumber;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

public class SingleNumberTest {
  protected Solution solution = new Solution();

  static Stream<Arguments> testDataProvider() {
    return Stream.of(
        Arguments.of(new int[] {2, 2, 1}, 1, "same numbers twice"),
        Arguments.of(new int[] {4, 1, 2, 1, 2}, 4, "same numbers interleaved"),
        Arguments.of(new int[] {1}, 1, "one element"));
  }

  @ParameterizedTest(name = "{2}")
  @MethodSource("testDataProvider")
  void testSingleNumber(int[] nums, int want, String displayName) {
    int actual = solution.singleNumber(nums);
    assertEquals(want, actual);

    int actual2 = solution.singleNumber2(nums);
    assertEquals(want, actual2);
  }
}
