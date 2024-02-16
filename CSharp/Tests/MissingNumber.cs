using Solutions;

namespace Tests;

public class MissingNumberTest {
    [Fact]
    public void Test() {
        List<(int[] nums, int want)> TestCases = [
            (nums: [3, 0, 1], want: 2), (nums: [0, 1], want: 2),
            (nums: [9, 6, 4, 2, 3, 5, 7, 0, 1], want: 8)
        ];

        foreach ((int[] nums, int want) in TestCases) {
            int result = MissingNumber.MissingNumber1(nums);
            Assert.Equal(want, result);
        }
    }
}