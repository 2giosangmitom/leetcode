using Solutions;
namespace Tests;

public class ValidParenthesesTest {
    [Fact]
    public void Test() {
        List<(string s, bool want)> TestCases = [
            (s: "[]", want: true),
            (s: "()[]{}", want: true),
            (s: "(]", want: false),
        ];

        foreach ((string s, bool want) in TestCases) {
            bool result = ValidParentheses.IsValid(s);
            Assert.Equal(want, result);
        }
    }
}