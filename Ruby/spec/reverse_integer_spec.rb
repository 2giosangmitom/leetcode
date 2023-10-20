# frozen_string_literal: true

require "reverse_integer"

describe "reverse integer" do
  cases = [
    { x: 123, want: 321 },
    { x: -123, want: -321 },
    { x: 120, want: 21 },
    { x: 1534236469, want: 0 },
    { x: -2147483648, want: 0 },
    { x: 900000, want: 9 },
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result = reverse(c[:x])
      expect(result).to eq(c[:want])
    end
  end
end
