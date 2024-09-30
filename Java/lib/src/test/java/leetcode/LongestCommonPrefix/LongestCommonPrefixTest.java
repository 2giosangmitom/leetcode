package leetcode.LongestCommonPrefix;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class LongestCommonPrefixTest {
  Solution solution = new Solution();

  static Stream<Arguments> longestCommonPrefixProvider() {
    return Stream.of(
        Arguments.of(new String[] {"flower", "flow", "flight"}, "fl"),
        Arguments.of(new String[] {"dog", "racecar", "car"}, ""));
  }

  @ParameterizedTest(name = "Case {index}: input = {0}, expected = \"{1}\"")
  @MethodSource("longestCommonPrefixProvider")
  void testLongestCommonPrefix(String[] input, String expected) {
    String result = solution.longestCommonPrefix(input);
    assertEquals(expected, result);
  }
}
