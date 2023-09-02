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

// Second solution
function numUniqueEmails2(emails: string[]): number {
  const hashSet = new Set<string>();
  for (const email of emails) {
    let cleanEmail = "";
    const part = email.split("@");
    const localName = part[0];
    const domainName = part[1];
    for (let i = 0; i < localName.length && localName[i] !== "+"; ++i) {
      if (localName[i] === ".") {
        continue;
      }
      cleanEmail += localName[i];
    }
    hashSet.add(cleanEmail + "@" + domainName);
  }
  return hashSet.size;
}

export { numUniqueEmails, numUniqueEmails2 };
