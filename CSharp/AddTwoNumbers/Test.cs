namespace CSharp.AddTwoNumbers;

public class Test
{
    [Fact]
    public void AddTwoNumbersTest()
    {
        List<(ListNode l1, ListNode l2, ListNode want)> TestCases =
            new()
            {
                (
                    l1: new ListNode(2, new ListNode(4, new ListNode(3))),
                    l2: new ListNode(5, new ListNode(6, new ListNode(4))),
                    want: new ListNode(7, new ListNode(0, new ListNode(8)))
                )
            };

        foreach ((ListNode l1, ListNode l2, ListNode want) in TestCases)
        {
            ListNode result = Solution.AddTwoNumbers(l1, l2);
            Assert.Equivalent(want, result);
        }
    }
}
