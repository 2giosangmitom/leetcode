package leetcode.LongestCommonPrefix;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class LongestCommonPrefixTest {
  Solution solution = new Solution();

  static Stream<Arguments> argsProvider() {
    return Stream.of(
        Arguments.of(new String[] {"flower", "flow", "flight"}, "fl"),
        Arguments.of(new String[] {"dog", "racecar", "car"}, ""));
  }

  @ParameterizedTest
  @MethodSource("argsProvider")
  void parameterizedTest(String[] strs, String want) {
    String result = solution.longestCommonPrefix(strs);
    assertEquals(want, result);
  }
}
