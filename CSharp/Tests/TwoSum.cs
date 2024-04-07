using Solutions;

namespace Tests;

public class TwoSumTest {
    [Fact]
    public void TestTwoSum_Case1() {
        int[] nums = [2, 7, 11, 15];
        int target = 9;
        int[] want = [0, 1];

        int[] result = TwoSum.TwoSum1(nums, target);
        int[] result2 = TwoSum.TwoSum2(nums, target);

        Assert.Equal(want, result);
        Assert.Equal(want, result2);
    }

    [Fact]
    public void TestTwoSum_Case2() {
        int[] nums = [3, 2, 4];
        int target = 6;
        int[] want = [1, 2];

        int[] result = TwoSum.TwoSum1(nums, target);
        int[] result2 = TwoSum.TwoSum2(nums, target);

        Assert.Equal(want, result);
        Assert.Equal(want, result2);
    }

    [Fact]
    public void TestTwoSum_Case3() {
        int[] nums = [3, 3];
        int target = 6;
        int[] want = [0, 1];

        int[] result = TwoSum.TwoSum1(nums, target);
        int[] result2 = TwoSum.TwoSum2(nums, target);

        Assert.Equal(want, result);
        Assert.Equal(want, result2);
    }

    [Fact]
    public void TestTwoSum_Case4() {
        int[] nums = [2, 3, 4, 1, 25, 8];
        int target = 30;
        int[] want = [-1];

        int[] result = TwoSum.TwoSum1(nums, target);
        int[] result2 = TwoSum.TwoSum2(nums, target);

        Assert.Equal(want, result);
        Assert.Equal(want, result2);
    }
}