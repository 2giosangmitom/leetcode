using Solutions;

namespace Tests;

public class ValidParenthesesTest {
    [Fact]
    public void TestValidParentheses_Case1() {
        string s = "[]";
        bool want = true;

        bool result = ValidParentheses.IsValid(s);
        Assert.Equal(want, result);
    }

    [Fact]
    public void TestValidParentheses_Case2() {
        string s = "()[]{}";
        bool want = true;

        bool result = ValidParentheses.IsValid(s);
        Assert.Equal(want, result);
    }

    [Fact]
    public void TestValidParentheses_Case3() {
        string s = "(]";
        bool want = false;

        bool result = ValidParentheses.IsValid(s);
        Assert.Equal(want, result);
    }
}
