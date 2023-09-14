package uniqueemailaddresses

import (
	"strings"
)

func numUniqueEmails(emails []string) int {
	uniqueEmails := []string{}
	for _, email := range emails {
		part := strings.Split(email, "@")
		local_name := part[0]
		domain_name := part[1]
		local_name = strings.ReplaceAll(local_name, ".", "")
		if strings.Contains(local_name, "+") {
			local_name, _, _ = strings.Cut(local_name, "+")
		}
		cleanEmail := local_name + "@" + domain_name
		if !func(email string) bool {
			for _, v := range uniqueEmails {
				if v == email {
					return true
				}
			}
			return false
		}(cleanEmail) {
			uniqueEmails = append(uniqueEmails, cleanEmail)
		}
	}
	return len(uniqueEmails)
}

// Second solution
func numUniqueEmails2(emails []string) int {
	hashSet := map[string]bool{}
	for _, email := range emails {
		cleanEmail := strings.Builder{}
		part := strings.Split(email, "@")
		localName := part[0]
		domainName := part[1]
		for i := 0; i < len(localName) && localName[i] != '+'; i++ {
			if localName[i] == '.' {
				continue
			}
			cleanEmail.WriteString(string(localName[i]))
		}
		hashSet[cleanEmail.String()+"@"+domainName] = true
	}
	return len(hashSet)
}
