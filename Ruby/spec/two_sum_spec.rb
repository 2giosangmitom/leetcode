# frozen_string_literal: true

require 'two_sum'

describe 'two_sum' do
  cases = [
    { nums: [2, 7, 11, 15], target: 9, want: [0, 1] },
    { nums: [3, 2, 4], target: 6, want: [1, 2] },
    { nums: [3, 3], target: 6, want: [0, 1] },
    { nums: [2, 3, 4, 1, 25, 8], target: 30, want: [-1] }
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result2 = two_sum2(c[:nums], c[:target])
      result  = two_sum(c[:nums], c[:target])
      expect(result).to eq(c[:want])
      expect(result2).to eq(c[:want])
    end
  end
end
