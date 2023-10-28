# frozen_string_literal: true

require 'reverse_integer'

describe 'reverse integer' do
  cases = [
    { x: 123, want: 321 },
    { x: -123, want: -321 },
    { x: 120, want: 21 },
    { x: 1_534_236_469, want: 0 },
    { x: -2_147_483_648, want: 0 },
    { x: 900_000, want: 9 }
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result = reverse(c[:x])
      expect(result).to eq(c[:want])
    end
  end
end
