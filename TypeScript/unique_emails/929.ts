function numUniqueEmails(emails: string[]): number {
  const uniqueEmails: string[] = [];
  for (const e of emails) {
    const part = e.split("@");
    let localName = part[0];
    const domainName = part[1];

    localName = localName.replaceAll(".", "");
    if (localName.includes("+")) {
      localName = localName.substring(0, localName.indexOf("+"));
    }
    const cleanEmail = localName + "@" + domainName;
    if (!uniqueEmails.includes(cleanEmail)) {
      uniqueEmails.push(cleanEmail);
    }
  }
  return uniqueEmails.length;
}

export default numUniqueEmails;
