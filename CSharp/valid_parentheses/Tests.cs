namespace valid_parenthese {
    public class Test_ValidParentheses {
        [Fact]
        public void Test1() {
            bool got = Solution.IsValid("()");
            Assert.True(got);
        }

        [Fact]
        public void Test2() {
            bool got = Solution.IsValid("()[]{}");
            Assert.True(got);
        }

        [Fact]
        public void Test3() {
            bool got = Solution.IsValid("(]");
            Assert.False(got);
        }

        [Fact]
        public void Test4() {
            bool got = Solution.IsValid("(})");
            Assert.False(got);
        }

        [Fact]
        public void Test5() {
            bool got = Solution.IsValid("{[]}");
            Assert.True(got);
        }

        [Fact]
        public void Test6() {
            bool got = Solution.IsValid("}}()");
            Assert.False(got);
        }
    }
}