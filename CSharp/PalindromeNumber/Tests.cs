namespace CSharp.PalindromeNumber;

public class Test {
    [Fact]
    public void PalindromeNumberTest() {
        List<(int num, bool want)> TestCases =
            new()
            {
                (num: -10, want: false),
                (num: 5, want: true),
                (num: 121, want: true),
                (num: 321, want: false),
                (num: 111, want: true)
            };

        foreach ((int num, bool want) in TestCases) {
            bool result = Solution.IsPalindrome(num);
            Assert.Equal(want, result);
        }
    }
}