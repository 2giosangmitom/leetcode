# frozen_string_literal: true

require 'palindrome_number'

describe 'palindrome number' do
  cases = [
    { num: -10, want: false },
    { num: 5, want: true },
    { num: 121, want: true },
    { num: 321, want: false },
    { num: 111, want: true }
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result = is_palindrome(c[:num])
      expect(result).to eq(c[:want])
    end
  end
end
