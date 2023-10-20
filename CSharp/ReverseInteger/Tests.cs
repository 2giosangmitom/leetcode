namespace CSharp.ReverseInteger;

public class Test {
    [Fact]
    public void ReverseIntegerTest() {
        List<(int x, int want)> TestCases =
            new()
            {
                (x: 123, want: 321),
                (x: -123, want: -321),
                (x: 120, want: 21),
                (x: 1534236469, want: 0),
                (x: -2147483648, want: 0),
                (x: 900000, want: 9)
            };

        foreach ((int x, int want) in TestCases) {
            int result = Solution.Reverse(x);
            Assert.Equal(want, result);
        }
    }
}