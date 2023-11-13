# frozen_string_literal: true

require 'longest_common_prefix'

describe 'longest common prefix' do
  cases = [
    { strs: %w[flower flow flight], want: 'fl' },
    { strs: %w[dog racecar car], want: '' },
    { strs: %w[chi chien chau], want: 'ch' }
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result = longest_common_prefix(c[:strs])
      expect(result).to eq(c[:want])
    end
  end
end
