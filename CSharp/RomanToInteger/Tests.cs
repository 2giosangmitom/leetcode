namespace CSharp.RomanToInteger;

public class Test {
    [Fact]
    public void RomanToIntegerTest() {
        List<(string roman, int want)> TestCases =
            new()
            {
                (roman: "III", want: 3),
                (roman: "LVIII", want: 58),
                (roman: "MCMXCIV", want: 1994),
                (roman: "XXIV", want: 24),
            };

        foreach ((string roman, int want) in TestCases) {
            int result = Solution.RomanToInt(roman);
            Assert.Equal(want, result);
        }
    }
}