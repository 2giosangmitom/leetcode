namespace longest_common_prefix;

public class Test_LongestCommonPrefix {
    [Fact]
    public void Test1() {
        string got = Solution.LongestCommonPrefix(new string[] { "flower", "flow", "flight" });
        Assert.Equal("fl", got);
    }

    [Fact]
    public void Test2() {
        string got = Solution.LongestCommonPrefix(new string[] { "dog", "racecar", "car" });
        Assert.Equal("", got);
    }
}
