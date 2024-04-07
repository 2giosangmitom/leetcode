using Solutions;

namespace Tests;

public class MissingNumberTest {
    [Fact]
    public void MissingNumber_Case1() {
        int[] nums = [3, 0, 1];
        int want = 2;

        int result = MissingNumber.MissingNumber1(nums);
        Assert.Equal(want, result);
    }

    [Fact]
    public void MissingNumber_Case2() {
        int[] nums = [0, 1];
        int want = 2;

        int result = MissingNumber.MissingNumber1(nums);
        Assert.Equal(want, result);
    }

    [Fact]
    public void MissingNumber_Case3() {
        int[] nums = [9, 6, 4, 2, 3, 5, 7, 0, 1];
        int want = 8;

        int result = MissingNumber.MissingNumber1(nums);
        Assert.Equal(want, result);
    }
}
