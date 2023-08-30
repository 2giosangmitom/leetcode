namespace unique_email_address {
    public class Test_UniqueEmailAddress {
        [Fact]
        public void Test1() {
            int got = Solution.NumUniqueEmails(new string[] { "test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com" });
            Assert.Equal(2, got);
        }

        [Fact]
        public void Test2() {
            int got = Solution.NumUniqueEmails(new string[] { "a@leetcode.com", "b@leetcode.com", "c@leetcode.com" });
            Assert.Equal(3, got);
        }
    }
}