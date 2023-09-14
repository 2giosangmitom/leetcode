namespace add_two_numbers;

public class Test_AddTwoNumbers {
    [Fact]
    public void Test1() {
        var l1 = new ListNode(2, new ListNode(4, new ListNode(3)));
        var l2 = new ListNode(5, new ListNode(6, new ListNode(4)));
        var want = new ListNode(7, new ListNode(0, new ListNode(8)));
        var got = Solution.AddTwoNumbers(l1, l2);
        Assert.Equivalent(want, got);
    }

    [Fact]
    public void Test2() {
        var l1 = new ListNode(0);
        var l2 = new ListNode(0);
        var want = new ListNode(0);
        var got = Solution.AddTwoNumbers(l1, l2);
        Assert.Equivalent(want, got);
    }
}
