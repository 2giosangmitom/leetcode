using Solutions;

namespace Tests;

public class SummaryRangesTest {
    [Fact]
    public void TestSummaryRanges_Case1() {
        int[] nums = [0, 1, 2, 4, 5, 7];
        string[] want = ["0->2", "4->5", "7"];

        IList<string> result = SummaryRanges.SummaryRanges1(nums);
        Assert.Equal(result, want);
    }

    [Fact]
    public void TestSummaryRanges_Case2() {
        int[] nums = [0, 2, 3, 4, 6, 8, 9];
        string[] want = ["0", "2->4", "6", "8->9"];

        IList<string> result = SummaryRanges.SummaryRanges1(nums);
        Assert.Equal(result, want);
    }
}