namespace add_two_numbers {
    public class Test_AddTwoNumbers {
        [Fact]
        public void Test1() {
            var l1 = new ListNode(2, new ListNode(4, new ListNode(3)));
            var l2 = new ListNode(5, new ListNode(6, new ListNode(4)));
            var want = new ListNode(7, new ListNode(0, new ListNode(8)));
            var result = Solution.AddTwoNumbers(l1, l2);
            Assert.Equivalent(want, result);
        }
    }
}