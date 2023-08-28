namespace remove_duplicates_from_sorted_array {
    public class Test_RmDupFromSortedArr {
        [Fact]
        public void Test1() {
            int[] arr = { 1, 1, 2 };
            int got = Solution.RemoveDuplicates(arr);
            Assert.Equal(2, got);
            Assert.Equal(new int[] { 1, 2 }, arr[..2]);
        }

        [Fact]
        public void Test2() {
            int[] arr = { 0, 0, 1, 1, 1, 2, 2, 3, 3, 4 };
            int got = Solution.RemoveDuplicates(arr);
            Assert.Equal(5, got);
            Assert.Equal(new int[] { 0, 1, 2, 3, 4 }, arr[..5]);
        }
    }
}