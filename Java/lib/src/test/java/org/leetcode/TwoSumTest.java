package org.leetcode;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import java.util.Arrays;

class TwoSumTest {
    private TwoSum twoSum;

    @BeforeEach
    void setUp() {
        twoSum = new TwoSum();
    }

    @Test
    @DisplayName("Contains two sum")
    void testExistingCuple() {
        int[] nums = { 2, 7, 11, 15 };
        int target = 9;
        int[] want = { 0, 1 };

        int[] result = twoSum.twoSum(nums, target);
        int[] result2 = twoSum.twoSum2(nums, target);
        assertArrayEquals(want, result);
        assertArrayEquals(want, result2);
    }

    @Test
    @DisplayName("Not contains two sum")
    void testNonExistingCuple() {
        int[] nums = { 2, 3, 4, 1, 25, 8 };
        int target = 30;
        int[] want = { -1 };

        int[] result = twoSum.twoSum(nums, target);
        int[] result2 = twoSum.twoSum2(nums, target);
        assertArrayEquals(want, result);
        assertArrayEquals(want, result2);
    }

    @ParameterizedTest(name = "Compute the two-sum for the target value {1} within the array [{0}]")
    @CsvSource({
            "'3, 2, 4', 6, '1, 2'",
            "'3, 3', 6, '0, 1'"
    })
    void testTwoSumVariants(String nums, int target, String want) {
        int[] arr = Arrays.stream(nums.split(", ")).mapToInt(Integer::parseInt).toArray();
        int[] wantArr = Arrays.stream(want.split(", ")).mapToInt(Integer::parseInt).toArray();
        int[] result = twoSum.twoSum(arr, target);
        int[] result2 = twoSum.twoSum2(arr, target);

        assertArrayEquals(wantArr, result);
        assertArrayEquals(wantArr, result2);
    }
}
