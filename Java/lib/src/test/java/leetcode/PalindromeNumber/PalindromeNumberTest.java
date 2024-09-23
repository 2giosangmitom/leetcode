package leetcode.PalindromeNumber;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class PalindromeNumberTest {
    Solution solution = new Solution();

    @ParameterizedTest
    @ValueSource(ints = { 121, 212 })
    void shouldReturnTrueForPalindromeNums(int number) {
        assertTrue(solution.isPalindrome(number));
    }

    @ParameterizedTest
    @ValueSource(ints = { 123, 10, -121 })
    void shouldReturnFalseForNonPalindromeNums(int number) {
        assertFalse(solution.isPalindrome(number));
    }
}
