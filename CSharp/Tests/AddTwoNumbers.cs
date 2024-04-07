using Solutions;

namespace Tests;

public class AddTwoNumbersTest {
    [Fact]
    public void AddTwoNumbers_Case1() {
        ListNode l1 = new(2, new ListNode(4, new ListNode(3)));
        ListNode l2 = new(5, new ListNode(6, new ListNode(4)));
        ListNode want = new(7, new ListNode(0, new ListNode(8)));

        ListNode result = AddTwoNumbers.AddTwoNumbers1(l1, l2);
        Assert.Equivalent(want, result);
    }

    [Fact]
    public void AddTwoNumbers_Case2() {
        ListNode l1 = new(0);
        ListNode l2 = new(0);
        ListNode want = new(0);

        ListNode result = AddTwoNumbers.AddTwoNumbers1(l1, l2);
        Assert.Equivalent(want, result);
    }

    [Fact]
    public void AddTwoNumbers_Case3() {
        ListNode l1 = new(
            9, new ListNode(
                9, new ListNode(
                    9, new ListNode(
                        9, new ListNode(
                            9, new ListNode(
                                9, new ListNode(9)))))));
        ListNode l2 = new(
            9, new ListNode(
                9, new ListNode(
                    9, new ListNode(9))));
        ListNode want = new(
            8, new ListNode(
                9, new ListNode(
                    9, new ListNode(
                        9, new ListNode(
                            0, new ListNode(
                                0, new ListNode(
                                    0, new ListNode(1))))))));

        ListNode result = AddTwoNumbers.AddTwoNumbers1(l1, l2);
        Assert.Equivalent(want, result);
    }
}
