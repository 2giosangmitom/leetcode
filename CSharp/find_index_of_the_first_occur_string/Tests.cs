namespace find_index_of_the_first_occur_string {
    public class Test_FindIndexOfTheFirstOccurString {
        [Fact]
        public void Test1() {
            int got = Solution.StrStr("sadbutsad", "sad");
            Assert.Equal(0, got);
        }

        [Fact]
        public void Test2() {
            int got = Solution.StrStr("leetcode", "leeto");
            Assert.Equal(-1, got);
        }

        [Fact]
        public void Test3() {
            int got = Solution.StrStr("hello", "ll");
            Assert.Equal(2, got);
        }

        [Fact]
        public void Test4() {
            int got = Solution.StrStr("a", "a");
            Assert.Equal(0, got);
        }

        [Fact]
        public void Test5() {
            int got = Solution.StrStr("abc", "c");
            Assert.Equal(2, got);
        }

        [Fact]
        public void Test6() {
            int got = Solution.StrStr("abbb", "aaaa");
            Assert.Equal(-1, got);
        }
    }
}
