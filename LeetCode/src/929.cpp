// Unique Email Addresses
#include <iostream>
#include <string.h>
#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
public:
  int numUniqueEmails(vector<string> &emails) {
    unordered_set<string> uniqueEmails;
    for (string &email : emails) {
      string cleanEmail;
      for (char c : email) {
        if (c == '+' || c == '@') {
          break;
        }
        if (c == '.') {
          continue;
        }
        cleanEmail += c;
      }
      cleanEmail += email.substr(email.find('@'));
      uniqueEmails.insert(cleanEmail);
    }
    return uniqueEmails.size();
  }
};