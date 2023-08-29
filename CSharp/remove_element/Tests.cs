namespace remove_element {
    public class Test_RemoveElement {
        [Fact]
        public void Test1() {
            int[] nums = { 0, 1, 2, 2, 3, 0, 4, 2 };
            int got = Solution.RemoveElement(nums, 2);
            Assert.Equal(5, got);
            Assert.Equal(new int[] { 0, 1, 3, 0, 4 }, nums[..5]);
        }

        [Fact]
        public void Test2() {
            int[] nums = { 0, 1, 4, 6, 7, 8, 9, 9, 22 };
            int got = Solution.RemoveElement(nums, 9);
            Assert.Equal(7, got);
            Assert.Equal(new int[] { 0, 1, 4, 6, 7, 8, 22 }, nums[..7]);
        }

        [Fact]
        public void Test3() {
            int[] nums = { 3, 2, 2, 3 };
            int got = Solution.RemoveElement(nums, 3);
            Assert.Equal(2, got);
            Assert.Equal(new int[] { 2, 2 }, nums[..2]);
        }
    }
}