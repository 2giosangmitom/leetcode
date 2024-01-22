using Solutions;

namespace Tests;

public class RomanToIntegerTest {
    [Fact]
    public void RomanToIntegerTest1() {
        List<(string roman, int want)> TestCases =
        [
            (roman: "III", want: 3),
            (roman: "LVIII", want: 58),
            (roman: "MCMXCIV", want: 1994),
            (roman: "XXIV", want: 24),
            (roman: "LLVMR", want: -1),
        ];

        foreach ((string roman, int want) in TestCases) {
            int result = RomanToInteger.RomanToInt(roman);
            Assert.Equal(want, result);
        }
    }
}