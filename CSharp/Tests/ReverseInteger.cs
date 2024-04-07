using Solutions;

namespace Tests;

public class ReverseIntegerTest {
    [Fact]
    public void ReverseInteger_Case1() {
        int x = 123;
        int want = 321;

        int result = ReverseInteger.Reverse(x);
        Assert.Equal(want, result);
    }

    [Fact]
    public void ReverseInteger_Case2() {
        int x = -123;
        int want = -321;

        int result = ReverseInteger.Reverse(x);
        Assert.Equal(want, result);
    }

    [Fact]
    public void ReverseInteger_Case3() {
        int x = 120;
        int want = 21;

        int result = ReverseInteger.Reverse(x);
        Assert.Equal(want, result);
    }

    [Fact]
    public void ReverseInteger_Case4() {
        int x = 1534236469;
        int want = 0;

        int result = ReverseInteger.Reverse(x);
        Assert.Equal(want, result);
    }

    [Fact]
    public void ReverseInteger_Case5() {
        int x = -2147483648;
        int want = 0;

        int result = ReverseInteger.Reverse(x);
        Assert.Equal(want, result);
    }

    [Fact]
    public void ReverseInteger_Case6() {
        int x = 900000;
        int want = 9;

        int result = ReverseInteger.Reverse(x);
        Assert.Equal(want, result);
    }
}
