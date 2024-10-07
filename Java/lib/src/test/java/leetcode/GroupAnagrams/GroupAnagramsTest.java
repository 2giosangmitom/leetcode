package leetcode.GroupAnagrams;

import java.util.Arrays;
import java.util.List;

import org.junit.jupiter.api.AssertionFailureBuilder;
import static org.junit.jupiter.api.AssertionFailureBuilder.assertionFailure;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class GroupAnagramsTest {

    protected Solution solution = new Solution();

    private void assertNestedList(List<List<String>> left, List<List<String>> right) {
        if (left.size() != right.size()) {
            AssertionFailureBuilder notSameSizeErr = assertionFailure();
            notSameSizeErr.message("two nested lists are not have same length");
            notSameSizeErr.buildAndThrow();
        }

        for (List<String> l : left) {
            boolean found = false;
            for (List<String> r : right) {
                if (l.containsAll(r)) {
                    found = true;
                    break;
                }
            }
            if (!found) {
                AssertionFailureBuilder notEqual = assertionFailure();
                notEqual.message("two nested lists are not equal");
                notEqual.actual(left);
                notEqual.expected(right);
                notEqual.buildAndThrow();
            }
        }
    }

    @Test
    @DisplayName("same length")
    void case1() {
        String[] strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
        List<List<String>> want = Arrays.asList(Arrays.asList("bat"), Arrays.asList("nat", "tan"), Arrays.asList("ate", "eat", "tea"));

        List<List<String>> actual = solution.groupAnagrams(strs);

        assertNestedList(actual, want);
    }
}
