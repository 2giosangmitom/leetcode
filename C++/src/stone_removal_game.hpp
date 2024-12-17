#ifndef STONE_REMOVAL_GAME_HPP
#define STONE_REMOVAL_GAME_HPP

class Solution {
public:
  bool canAliceWin(int n) {
    int count = 0;
    int i = 10;
    while (n - i >= 0) {
      n -= i;
      i--;
      count++;
    }
    return count % 2 != 0;
  };
};

#endif // STONE_REMOVAL_GAME_HPP