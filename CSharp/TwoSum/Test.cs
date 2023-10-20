namespace CSharp.TwoSum;

public class Test
{
    [Fact]
    public void TwoSumTest()
    {
        List<(int[] nums, int target, int[] want)> TestCases =
            new()
            {
                (nums: new int[] { 2, 7, 11, 15 }, target: 9, want: new int[] { 0, 1 }),
                (nums: new int[] { 3, 2, 4 }, target: 6, want: new int[] { 1, 2 }),
                (nums: new int[] { 3, 3 }, target: 6, want: new int[] { 0, 1 }),
                (nums: new int[] { 2, 3, 4, 1, 25, 8 }, target: 30, want: new int[] { -1 }),
            };

        foreach ((int[] nums, int target, int[] want) in TestCases)
        {
            int[] result = Solution.TwoSum(nums, target);
            int[] result2 = Solution.TwoSum2(nums, target);
            Assert.Equal(want, result);
            Assert.Equal(want, result2);
        }
    }
}
