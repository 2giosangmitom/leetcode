namespace Solutions;

public class RomanToInteger
{
    public static int RomanToInt(string s)
    {
        Dictionary<char, int> romanMap =
            new()
            {
                { 'I', 1 },
                { 'V', 5 },
                { 'X', 10 },
                { 'L', 50 },
                { 'C', 100 },
                { 'D', 500 },
                { 'M', 1000 },
            };

        int result = 0;

        foreach (char c in s.Reverse())
        {
            if (romanMap.TryGetValue(c, out int value))
            {
                int number = value;
                if (value * 4 < result)
                {
                    result -= number;
                }
                else
                {
                    result += number;
                }
            }
            else
            {
                return -1;
            }
        }

        return result;
    }
}
