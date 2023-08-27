namespace roman_2_int {
    public class Test_Roman2Int {
        [Fact]
        public void Test1() {
            int got = Solution.RomanToInt("III");
            Assert.Equal(3, got);
        }

        [Fact]
        public void Test2() {
            int got = Solution.RomanToInt("LVIII");
            Assert.Equal(58, got);
        }

        [Fact]
        public void Test3() {
            int got = Solution.RomanToInt("MCMXCIV");
            Assert.Equal(1994, got);
        }
    }
}