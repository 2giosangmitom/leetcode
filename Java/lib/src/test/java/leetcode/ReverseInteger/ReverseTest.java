package leetcode.ReverseInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ReverseTest {
    Solution solution = new Solution();

    @ParameterizedTest
    @CsvSource({
            "123, 321",
            "-123, -321",
            "120, 21",
            "1463847412, 2147483641",
            "-2147483648, 0",
    })
    void manyTests(int x, int want) {
        int result = solution.reverse(x);
        assertEquals(result, want);
    }
}
