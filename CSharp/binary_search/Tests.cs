namespace binary_search;

public class Test_BinarySearch {
    [Fact]
    public void Test1() {
        int got = Solution.Search(new int[] { -1, 0, 3, 5, 9, 12 }, 9);
        Assert.Equal(4, got);
    }

    [Fact]
    public void Test2() {
        int got = Solution.Search(new int[] { -1, 0, 3, 5, 9, 12 }, 2);
        Assert.Equal(-1, got);
    }
}
