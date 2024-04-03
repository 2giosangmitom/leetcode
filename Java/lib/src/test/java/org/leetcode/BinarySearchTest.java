package org.leetcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class BinarySearchTest {
    @Test
    void Test1() {
        Solution s = new Solution();
        assertEquals(4, s.search(new int[] { -1, 0, 3, 5, 9, 12 }, 9));
    }

    @Test
    void Test2() {
        Solution s = new Solution();
        assertEquals(-1, s.search(new int[] { -1, 0, 3, 5, 9, 12 }, 2));
    }
}
