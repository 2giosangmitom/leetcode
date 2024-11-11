package leetcode.GroupAnagrams;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

class Solution {

  static String sortString(String s) {
    char arr[] = s.toCharArray();
    Arrays.sort(arr);
    return new String(arr);
  }

  public List<List<String>> groupAnagrams(String[] strs) {
    HashMap<String, ArrayList<String>> map = new HashMap<>();

    for (String s : strs) {
      String key = sortString(s);
      ArrayList<String> group = map.get(key);
      if (group != null) {
        group.addLast(s);
      } else {
        ArrayList<String> new_group = new ArrayList<>();
        String new_key = sortString(s);
        new_group.addLast(s);
        map.put(new_key, new_group);
      }
    }

    List<List<String>> result = new ArrayList<>();
    map.forEach(
        (_k, v) -> {
          result.add(v);
        });

    return result;
  }
}
