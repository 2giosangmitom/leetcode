package leetcode.BestTimeToBuyAndSellStock;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

public class BestTimeToBuyAndSellStockTest {
  protected Solution solution = new Solution();

  @Test
  @DisplayName("random order prices")
  void testBestTimeToBuyAndSellStock_case1() {
    int prices[] = {7, 1, 5, 3, 6, 4};
    int expected = 5;
    int actual = solution.maxProfit(prices);
    assertEquals(expected, actual);
  }

  @Test
  @DisplayName("descending order prices")
  void testBestTimeToBuyAndSellStock_case2() {
    int prices[] = {7, 6, 4, 3, 1};
    int expected = 0;
    int actual = solution.maxProfit(prices);
    assertEquals(expected, actual);
  }

  @Test
  @DisplayName("ascending order prices")
  void testBestTimeToBuyAndSellStock_case3() {
    int prices[] = {1, 2, 3, 4, 5};
    int expected = 4;
    int actual = solution.maxProfit(prices);
    assertEquals(expected, actual);
  }
}
