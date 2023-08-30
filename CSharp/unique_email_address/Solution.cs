namespace unique_email_address {
    public class Solution {
        public static int NumUniqueEmails(string[] emails) {
            var cleanEmails = new List<string>();
            foreach (var email in emails) {
                string[] part = email.Split('@');
                string localName = part[0];
                string domainName = part[1];
                localName = localName.Replace(".", "");
                if (localName.Contains('+')) {
                    localName = localName[..localName.IndexOf("+")];
                }
                string cleanEmail = localName + "@" + domainName;
                if (!cleanEmails.Contains(cleanEmail)) {
                    cleanEmails.Add(cleanEmail);
                }
            }
            return cleanEmails.Count;
        }
    }
}