package org.leetcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class BinarySearchTest {
    @Test
    void Test() {
        Solution s = new Solution();
        assertEquals(4, s.search(new int[] { -1, 0, 3, 5, 9, 12 }, 9));
    }
}
