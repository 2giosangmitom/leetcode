namespace remove_element {
    public class Test_RemoveElement {
        [Fact]
        public void Test1() {
            int[] nums = { 0, 1, 2, 2, 3, 0, 4, 2 };
            int got = Solution.RemoveElement(nums, 2);
            Assert.Equal(5, got);
            Assert.Equal(new int[] { 0, 1, 3, 0, 4 }, nums[..5]);
        }
    }
}