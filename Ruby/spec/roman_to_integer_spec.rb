# frozen_string_literal: true

require "roman_to_integer"

describe 'roman to integer' do
  cases = [
    { roman: "III", want: 3 },
    { roman: "LVIII", want: 58 },
    { roman: "MCMXCIV", want: 1994 },
    { roman: "XXIV", want: 24 },
    { roman: "LLVMR", want: -1 },
  ]

  cases.each_with_index do |c, i|
    it "case #{i}" do
      result = roman_to_int(c[:roman])
      expect(result).to eq(c[:want])
    end
  end
end
