import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class ReverseIntegerTests {
  @ParameterizedTest(name = "reverse({0})")
  @CsvSource({"123,321", "-123,-321", "120,21", "1534236469,0", "-2147483648,0", "900000,9"})
  void test(int x, int want) {
    int result = ReverseInteger.reverse(x);
    assertEquals(want, result);
  }
}
