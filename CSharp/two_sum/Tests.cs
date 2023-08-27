namespace two_sum {
    public class Test_TwoSum {
        [Fact]
        public void Test1() {
            int[] got = Solution.TwoSum(new int[] { 2, 7, 11, 15 }, 9);
            Assert.Equal(new int[] { 0, 1 }, got);
        }

        [Fact]
        public void Test2() {
            int[] got = Solution.TwoSum(new int[] { 3, 2, 4 }, 6);
            Assert.Equal(new int[] { 1, 2 }, got);
        }
    }
}