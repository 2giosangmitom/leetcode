namespace Solutions;

public class LongestCommonPrefix
{
    public static string LongestCommonPrefix1(string[] strs)
    {
        string prefix = strs.First();

        for (int i = 1; i < strs.Length; i++)
        {
            while (!strs[i].StartsWith(prefix, StringComparison.Ordinal))
            {
                prefix = prefix[..(prefix.Length - 1)];
                if (prefix.Length == 0)
                {
                    return "";
                }
            }
        }

        return prefix;
    }
}
