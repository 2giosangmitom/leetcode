namespace roman_2_int;

public class Solution {
    public static int RomanToInt(string s) {
        var (result, number) = (0, 0);
        for (int i = s.Length - 1; i >= 0; --i) {
            switch (s[i]) {
                case 'I':
                    number = 1;
                    break;
                case 'V':
                    number = 5;
                    break;
                case 'X':
                    number = 10;
                    break;
                case 'L':
                    number = 50;
                    break;
                case 'C':
                    number = 100;
                    break;
                case 'D':
                    number = 500;
                    break;
                case 'M':
                    number = 1000;
                    break;
            }
            if (number * 4 < result) {
                result -= number;
            } else {
                result += number;
            }
        }
        return result;
    }
}
