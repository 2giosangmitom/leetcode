namespace length_of_last_word {
    public class Test_LengthOfLastWord {
        [Fact]
        public void Test1() {
            int got = Solution.LengthOfLastWord("Hello World");
            Assert.Equal(5, got);
        }

        [Fact]
        public void Test2() {
            int got = Solution.LengthOfLastWord("   fly me   to   the moon  ");
            Assert.Equal(4, got);
        }

        [Fact]
        public void Test3() {
            int got = Solution.LengthOfLastWord("luffy is still joyboy");
            Assert.Equal(6, got);
        }
    }
}