import 'package:test/test.dart';
import 'package:leetcode/two_sum.dart';

typedef TestList = List<({List<int> nums, int target, List<int> want})>;

void main() {
  test('two sum', () {
    TestList cases = [
      (nums: [2, 7, 11, 15], target: 9, want: [0, 1]),
      (nums: [3, 2, 4], target: 6, want: [1, 2]),
      (nums: [3, 3], target: 6, want: [0, 1]),
      (nums: [2, 3, 4, 1, 25, 8], target: 30, want: [-1])
    ];

    for (var tt in cases) {
      List<int> result = Solution().twoSum(tt.nums, tt.target);
      List<int> result2 = Solution().twoSum2(tt.nums, tt.target);
      expect(result, equals(tt.want));
      expect(result2, equals(tt.want));
    }
  });
}
