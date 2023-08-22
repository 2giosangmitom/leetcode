/**
 * Runtime: 62ms (Beats 51.8%)
 * Memory: 43MB (Beats 41.44%)
 */

function numIdenticalPairs(nums: number[]): number {
  let result = 0;
  const hashMap = new Map<number, number>();
  for (const num of nums) {
    if (!hashMap.has(num)) {
      hashMap.set(num, 1);
    } else {
      result += hashMap.get(num) ?? 0;
      hashMap.set(num, (hashMap.get(num) ?? 0) + 1);
    }
  }
  return result;
}

export default numIdenticalPairs;
