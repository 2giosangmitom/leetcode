namespace num_of_good_pairs {
    public class Test_NumberOfGoodPairs {
        [Fact]
        public void Test1() {
            int got = Solution.NumIdenticalPairs(new int[] { 1, 2, 3, 1, 1, 3 });
            Assert.Equal(4, got);
        }

        [Fact]
        public void Test2() {
            int got = Solution.NumIdenticalPairs(new int[] { 1, 1, 1, 1 });
            Assert.Equal(6, got);
        }

        [Fact]
        public void Test3() {
            int got = Solution.NumIdenticalPairs(new int[] { 1, 2, 3 });
            Assert.Equal(0, got);
        }
    }
}