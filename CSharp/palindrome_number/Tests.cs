namespace palindrome_number {
    public class Test_PalindromeNumber {
        [Fact]
        public void Test1() {
            bool got = Solution.IsPalindrome(121);
            Assert.True(got);
        }

        [Fact]
        public void Test2() {
            bool got = Solution.IsPalindrome(-121);
            Assert.False(got);
        }

        [Fact]
        public void Test3() {
            bool got = Solution.IsPalindrome(10);
            Assert.False(got);
        }

        [Fact]
        public void Test4() {
            bool got = Solution.IsPalindrome(0);
            Assert.True(got);
        }
    }
}