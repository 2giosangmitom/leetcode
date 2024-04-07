using Solutions;

namespace Tests;

public class PalindromeNumberTest {
    [Fact]
    public void PalindromeNumber_Case1() {
        int num = -10;
        bool want = false;

        bool result = PalindromeNumber.IsPalindrome(num);
        Assert.Equal(want, result);
    }

    [Fact]
    public void PalindromeNumber_Case2() {
        int num = 5;
        bool want = true;

        bool result = PalindromeNumber.IsPalindrome(num);
        Assert.Equal(want, result);
    }

    [Fact]
    public void PalindromeNumber_Case3() {
        int num = 121;
        bool want = true;

        bool result = PalindromeNumber.IsPalindrome(num);
        Assert.Equal(want, result);
    }

    [Fact]
    public void PalindromeNumber_Case4() {
        int num = 321;
        bool want = false;

        bool result = PalindromeNumber.IsPalindrome(num);
        Assert.Equal(want, result);
    }

    [Fact]
    public void PalindromeNumber_Case5() {
        int num = 111;
        bool want = true;

        bool result = PalindromeNumber.IsPalindrome(num);
        Assert.Equal(want, result);
    }
}
