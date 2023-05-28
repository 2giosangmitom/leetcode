// Unique Email Addresses
const numUniqueEmails = function (emails) {
	const uniqueEmails = [];
	emails.forEach((email) => {
		const part = email.split("@");
		part[0] = part[0].replace(/\./g, "");
		if (part[0].includes("+")) {
			part[0] = part[0].substring(0, part[0].indexOf("+"));
		}
		const cleanEmail = part.join("@");
		if (!uniqueEmails.includes(cleanEmail)) {
			uniqueEmails.push(cleanEmail);
		}
	});
	return uniqueEmails.length;
};
