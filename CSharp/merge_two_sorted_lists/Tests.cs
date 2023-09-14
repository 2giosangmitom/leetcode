namespace merge_two_sorted_lists;

public class Test_MergeTwoSortedLists {
    [Fact]
    public void Test1() {
        var list1 = new ListNode(1, new ListNode(2, new ListNode(4)));
        var list2 = new ListNode(1, new ListNode(3, new ListNode(4)));
        var got = Solution.MergeTwoLists(list1, list2);
        var want = new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4))))));
        Assert.Equivalent(want, got);
    }
}
