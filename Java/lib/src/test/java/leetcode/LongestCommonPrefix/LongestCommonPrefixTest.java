package leetcode.LongestCommonPrefix;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class LongestCommonPrefixTest {
  protected Solution solution = new Solution();

  static Stream<Arguments> provideTestCases() {
    return Stream.of(
        Arguments.of("common prefix", new String[] {"flower", "flow", "flight"}, "fl"),
        Arguments.of("no common prefix", new String[] {"dog", "racecar", "car"}, ""),
        Arguments.of("empty string array", new String[] {}, ""),
        Arguments.of("single string", new String[] {"single"}, "single"),
        Arguments.of("all strings are the same", new String[] {"test", "test", "test"}, "test"),
        Arguments.of("mixed case strings", new String[] {"a", "A"}, ""),
        Arguments.of("one empty string", new String[] {"", "b"}, ""));
  }

  @ParameterizedTest(name = "{0}")
  @MethodSource("provideTestCases")
  void testLongestCommonPrefix(String name, String[] strs, String want) {
    String actual = solution.longestCommonPrefix(strs);
    assertEquals(want, actual);
  }
}
