namespace water_bottles;

public class Solution {
  public static int NumWaterBottles(int numBottles, int numExchange) {
    int drunk = 0;
    while (numBottles >= numExchange) {
      int remainder = numBottles % numExchange;
      drunk += numBottles - remainder;
      numBottles = numBottles / numExchange + remainder;
    }
    return drunk + numBottles;
  }
}
