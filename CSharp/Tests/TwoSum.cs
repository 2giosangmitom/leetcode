using Solutions;

namespace Tests;

public class TwoSumTest {
    [Fact]
    public void Test() {
        List<(int[] nums, int target, int[] want)> TestCases =
        [
            (nums: [2, 7, 11, 15], target: 9, want: [0, 1]),
            (nums: [3, 2, 4], target: 6, want: [1, 2]),
            (nums: [3, 3], target: 6, want: [0, 1]),
            (nums: [2, 3, 4, 1, 25, 8], target: 30, want: [-1]),
        ];

        foreach ((int[] nums, int target, int[] want) in TestCases) {
            int[] result = TwoSum.TwoSum1(nums, target);
            int[] result2 = TwoSum.TwoSum2(nums, target);
            Assert.Equal(want, result);
            Assert.Equal(want, result2);
        }
    }
}