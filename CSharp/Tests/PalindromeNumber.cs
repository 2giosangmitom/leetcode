using Solutions;

namespace Tests;

public class PalindromeNumberTest {
    [Fact]
    public void Test() {
        List<(int num, bool want)> TestCases =
        [
            (num: -10, want: false),
            (num: 5, want: true),
            (num: 121, want: true),
            (num: 321, want: false),
            (num: 111, want: true)
        ];

        foreach ((int num, bool want) in TestCases) {
            bool result = PalindromeNumber.IsPalindrome(num);
            Assert.Equal(want, result);
        }
    }
}