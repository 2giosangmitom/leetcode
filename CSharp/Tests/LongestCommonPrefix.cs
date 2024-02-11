using Solutions;

namespace Tests;

public class LongestCommonPrefixTest {
    [Fact]
    public void Test() {
        List<(string[] strs, string want)> TestCases = [
            (new string[] { "flower", "flow", "flight" }, "fl"),
            (new string[] { "dog", "racecar", "car" }, ""),
            (new string[] { "chi", "chien", "chau" }, "ch")
        ];

        foreach ((string[] strs, string want) in TestCases) {
            string result = LongestCommonPrefix.LongestCommonPrefix1(strs);
            Assert.Equal(want, result);
        }
    }
}