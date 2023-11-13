# frozen_string_literal: true

# @param {String[]} strs
# @return {String}
def longest_common_prefix(strs)
  prefix = strs[0]
  for i in 1...strs.length
    while strs[i].index(prefix) != 0
      prefix = prefix[0, prefix.length - 1]
      return '' if prefix.empty?
    end
  end
  prefix
end
