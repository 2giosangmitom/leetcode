namespace search_insert_pos;

public class Test_SearchInsertPosition {
  [Fact]
  public void Test1() {
    int got = Solution.SearchInsert(new int[] { 1, 3, 5, 6 }, 5);
    Assert.Equal(2, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.SearchInsert(new int[] { 1, 3, 5, 6 }, 2);
    Assert.Equal(1, got);
  }

  [Fact]
  public void Test3() {
    int got = Solution.SearchInsert(new int[] { 1, 3, 5, 6 }, 7);
    Assert.Equal(4, got);
  }
}
