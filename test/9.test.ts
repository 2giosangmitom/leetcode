import isPalindrome from "../src/9";
import { describe, expect, test } from "@jest/globals";

describe("Palindrome Number", () => {
  test("case 1", () => {
    const got = isPalindrome(-10);
    expect(got).toBe(false);
  });

  test("case 2", () => {
    const got = isPalindrome(121);
    expect(got).toBe(true);
  });

  test("case 3", () => {
    const got = isPalindrome(10);
    expect(got).toBe(false);
  });

  test("case 4", () => {
    const got = isPalindrome(5);
    expect(got).toBe(true);
  });
});
