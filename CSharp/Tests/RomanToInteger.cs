using Solutions;

namespace Tests;

public class RomanToIntegerTest {
    [Fact]
    public void RomanToIntegerTest_Case1() {
        string roman = "III";
        int want = 3;

        int result = RomanToInteger.RomanToInt(roman);
        Assert.Equal(want, result);
    }

    [Fact]
    public void RomanToIntegerTest_Case2() {
        string roman = "LVIII";
        int want = 58;

        int result = RomanToInteger.RomanToInt(roman);
        Assert.Equal(want, result);
    }

    [Fact]
    public void RomanToIntegerTest_Case3() {
        string roman = "MCMXCIV";
        int want = 1994;

        int result = RomanToInteger.RomanToInt(roman);
        Assert.Equal(want, result);
    }

    [Fact]
    public void RomanToIntegerTest_Case4() {
        string roman = "XXIV";
        int want = 24;

        int result = RomanToInteger.RomanToInt(roman);
        Assert.Equal(want, result);
    }

    [Fact]
    public void RomanToIntegerTest_Case5() {
        string roman = "LLVMR";
        int want = -1;

        int result = RomanToInteger.RomanToInt(roman);
        Assert.Equal(want, result);
    }
}
