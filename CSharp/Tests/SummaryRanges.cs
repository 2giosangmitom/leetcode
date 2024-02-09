using Solutions;

namespace Tests;

public class SummaryRangesTest {
    [Fact]
    public void Test() {
        List<(int[] nums, string[] want)> TestCases = [
            (nums: [0, 1, 2, 4, 5, 7], want: ["0->2", "4->5", "7"]),
            (nums: [0, 2, 3, 4, 6, 8, 9], want: ["0", "2->4", "6", "8->9"]),
        ];

        foreach ((int[] nums, string[] want) in TestCases) {
            IList<string> result = SummaryRanges.SummaryRanges1(nums);
            Assert.Equal(want, result);
        }
    }
}