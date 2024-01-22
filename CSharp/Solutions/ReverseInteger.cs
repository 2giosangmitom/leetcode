namespace Solutions;

public class ReverseInteger
{
    public static int Reverse(int x)
    {
        int result = 0;

        while (x != 0)
        {
            int lastDigit = x % 10;

            try
            {
                result = checked((result * 10) + lastDigit);
            }
            catch (OverflowException)
            {
                return 0;
            }

            x /= 10;
        }

        return result;
    }
}
