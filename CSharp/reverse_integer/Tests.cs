namespace reverse_integer {
    public class Test_ReverseInteger {
        [Fact]
        public void Test1() {
            int got = Solution.Reverse(123);
            Assert.Equal(321, got);
        }

        [Fact]
        public void Test2() {
            int got = Solution.Reverse(-123);
            Assert.Equal(-321, got);
        }

        [Fact]
        public void Test3() {
            int got = Solution.Reverse(1534236469);
            Assert.Equal(0, got);
        }

        [Fact]
        public void Test4() {
            int got = Solution.Reverse(900000);
            Assert.Equal(9, got);
        }
    }
}