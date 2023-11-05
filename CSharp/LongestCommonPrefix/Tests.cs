namespace CSharp.LongestCommonPrefix;

public class Test {
    [Fact]
    public void LongestCommonPrefixTest() {
        List<(string[] strs, string want)> TestCases =
            new()
            {
                (new string[]{ "flower","flow","flight"}, "fl"),
                (new string[]{"dog", "racecar", "car"}, ""),
                (new string[]{"chi", "chien", "chau"}, "ch")
            };

        foreach ((string[] strs, string want) in TestCases) {
            string result = Solution.LongestCommonPrefix(strs);
            Assert.Equal(want, result);
        }
    }
}