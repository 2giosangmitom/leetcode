using Solutions;

namespace Tests;

public class SingleNumberTest {
    [Fact]
    public void SingleNumber_Case1() {
        int[] nums = [2, 2, 1];
        int want = 1;

        int result = SingleNumber.SingleNumber1(nums);
        Assert.Equal(want, result);
    }

    [Fact]
    public void SingleNumber_Case2() {
        int[] nums = [4, 1, 2, 1, 2];
        int want = 4;

        int result = SingleNumber.SingleNumber1(nums);
        Assert.Equal(want, result);
    }

    [Fact]
    public void SingleNumber_Case3() {
        int[] nums = [1];
        int want = 1;

        int result = SingleNumber.SingleNumber1(nums);
        Assert.Equal(want, result);
    }
}
