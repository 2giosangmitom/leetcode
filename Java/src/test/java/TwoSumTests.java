import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class TwoSumTests {
  @Test
  @DisplayName("case 1")
  void test_1() {
    int[] result = TwoSum.twoSum(new int[] {2, 7, 11, 15}, 9);
    assertArrayEquals(new int[] {0, 1}, result);
  }

  @Test
  @DisplayName("case 2")
  void test_2() {
    int[] result = TwoSum.twoSum(new int[] {3, 2, 4}, 6);
    assertArrayEquals(new int[] {1, 2}, result);
  }

  @Test
  @DisplayName("case 3")
  void test_3() {
    int[] result = TwoSum.twoSum(new int[] {3, 3}, 6);
    assertArrayEquals(new int[] {0, 1}, result);
  }

  @Test
  @DisplayName("case 4")
  void test_4() {
    int[] result = TwoSum.twoSum(new int[] {2, 3, 4, 1, 25, 8}, 30);
    assertArrayEquals(new int[] {-1}, result);
  }
}
