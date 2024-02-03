using Solutions;

namespace Tests;

public class SingleNumberTest {
    [Fact]
    public void SingleNumberTest1() {
        List<(int[] nums, int want)> TestCases = [
            (nums: [2, 2, 1], want: 1),
            (nums: [4, 1, 2, 1, 2], want: 4),
            (nums: [1], want: 1),
        ];

        foreach ((int[] nums, int want) in TestCases) {
            int result = SingleNumber.SingleNumber1(nums);
            Assert.Equal(want, result);
        }
    }
}