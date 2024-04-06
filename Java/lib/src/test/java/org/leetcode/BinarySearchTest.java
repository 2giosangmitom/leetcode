package org.leetcode;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;

class BinarySearchTest {
    private Solution solution;

    @BeforeEach
    void setUp() {
        solution = new Solution();
    }

    @Test
    @DisplayName("Search for existing element")
    void testExistingElement() {
        assertEquals(4, solution.search(new int[] { -1, 0, 3, 5, 9, 12 }, 9));
    }

    @Test
    @DisplayName("Search for non-existing element")
    void testNonExistingElement() {
        assertEquals(-1, solution.search(new int[] { -1, 0, 3, 5, 9, 12 }, 2));
    }

    @ParameterizedTest(name = "Search for {1} in array [{2}]")
    @CsvSource({
            "-1, -1, '0, 3, 5, 9, 12'",
            "3, 9, '0, 3, 5, 9, 12'"
    })
    void testSearchVariants(int expectedIndex, int target, String array) {
        int[] arr = Arrays.stream(array.split(", ")).mapToInt(Integer::parseInt).toArray();
        assertEquals(expectedIndex, solution.search(arr, target));
    }
}
