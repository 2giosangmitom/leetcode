package leetcode.GroupAnagrams;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class GroupAnagramsTest {
  protected Solution solution = new Solution();

  private static Stream<Arguments> provideTestCases() {
    return Stream.of(
        Arguments.of(
            new String[] {"eat", "tea", "tan", "ate", "nat", "bat"},
            new String[][] {{"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}},
            "same length"),
        Arguments.of(new String[] {""}, new String[][] {{""}}, "empty string"),
        Arguments.of(
            new String[] {"a", "b", "a"},
            new String[][] {{"a", "a"}, {"b"}},
            "single character strings"),
        Arguments.of(new String[] {"a", "A"}, new String[][] {{"a"}, {"A"}}, "mixed case strings"),
        Arguments.of(
            new String[] {"abc", "def", "ghi"},
            new String[][] {{"abc"}, {"def"}, {"ghi"}},
            "no anagrams"),
        Arguments.of(
            new String[] {"abc", "bca", "cab"},
            new String[][] {{"abc", "bca", "cab"}},
            "all anagrams"));
  }

  @ParameterizedTest(name = "{2}")
  @MethodSource("provideTestCases")
  void testGroupAnagrams(String[] strs, String[][] want, String displayName) {
    List<List<String>> actual = solution.groupAnagrams(strs);

    List<List<String>> expected =
        Arrays.stream(want).map(Arrays::asList).collect(Collectors.toList());

    actual.forEach(Collections::sort);
    expected.forEach(Collections::sort);
    actual.sort(Comparator.<List<String>>comparingInt(List::size).thenComparing(List::getFirst));
    expected.sort(Comparator.<List<String>>comparingInt(List::size).thenComparing(List::getFirst));

    assertEquals(expected, actual);
  }
}
