package leetcode.SummaryRanges;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import java.util.Arrays;
import java.util.List;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class SummaryRangesTest {
  protected Solution solution = new Solution();

  @Test
  @DisplayName("short ranges")
  void test1() {
    int[] nums = {0, 1, 2, 4, 5, 7};
    List<String> want = Arrays.asList("0->2", "4->5", "7");
    List<String> actual = solution.summaryRanges(nums);
    assertArrayEquals(want.toArray(), actual.toArray());
  }

  @Test
  @DisplayName("long ranges")
  void test2() {
    int[] nums = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    List<String> want = Arrays.asList("0->9");
    List<String> actual = solution.summaryRanges(nums);
    assertArrayEquals(want.toArray(), actual.toArray());
  }

  @Test
  @DisplayName("empty ranges")
  void test3() {
    int[] nums = {};
    List<String> want = Arrays.asList();
    List<String> actual = solution.summaryRanges(nums);
    assertArrayEquals(want.toArray(), actual.toArray());
  }

  @Test
  @DisplayName("single ranges")
  void test4() {
    int[] nums = {0, 2, 4, 6, 8};
    List<String> want = Arrays.asList("0", "2", "4", "6", "8");
    List<String> actual = solution.summaryRanges(nums);
    assertArrayEquals(want.toArray(), actual.toArray());
  }

  @Test
  @DisplayName("double ranges")
  void test5() {
    int[] nums = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11};
    List<String> want = Arrays.asList("0->9", "11");
    List<String> actual = solution.summaryRanges(nums);
    assertArrayEquals(want.toArray(), actual.toArray());
  }
}
