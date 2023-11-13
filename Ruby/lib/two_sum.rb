# frozen_string_literal: true

# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  map = {}
  nums.each_with_index do |num, i|
    complement = target - num
    return [map[complement], i] if map.key?(complement)

    map[num] = i
  end
  [-1]
end

# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum2(nums, target)
  nums.each_with_index do |num, i|
    complement = target - num

    for j in i + 1...nums.length
      return [i, j] if complement == nums[j]
    end
  end
  [-1]
end
