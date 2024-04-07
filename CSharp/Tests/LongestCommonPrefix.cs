using Solutions;

namespace Tests;

public class LongestCommonPrefixTest {
    [Fact]
    public void LongestCommonPrefix_Case1() {
        string[] strs = ["flower", "flow", "flight"];
        string want = "fl";

        string result = LongestCommonPrefix.LongestCommonPrefix1(strs);
        Assert.Equal(want, result);
    }

    [Fact]
    public void LongestCommonPrefix_Case2() {
        string[] strs = ["dog", "racecar", "car"];
        string want = "";

        string result = LongestCommonPrefix.LongestCommonPrefix1(strs);
        Assert.Equal(want, result);
    }

    [Fact]
    public void LongestCommonPrefix_Case3() {
        string[] strs = ["chi", "chien", "chau"];
        string want = "ch";

        string result = LongestCommonPrefix.LongestCommonPrefix1(strs);
        Assert.Equal(want, result);
    }
}
