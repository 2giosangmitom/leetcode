import twoSum from "../src/1";
import { describe, expect, test } from "@jest/globals";

describe("Two Sum", () => {
  test("case 1", () => {
    const got = twoSum([2, 7, 11, 15], 9);
    expect(got).toEqual([0, 1]);
  });

  test("case 2", () => {
    const got = twoSum([3, 2, 4], 6);
    expect(got).toEqual([1, 2]);
  });

  test("case 3", () => {
    const got = twoSum([3, 3], 6);
    expect(got).toEqual([0, 1]);
  });

  test("case 4", () => {
    const got = twoSum([2, 3, 4, 1, 25, 8], 30);
    expect(got).toEqual([-1]);
  });
});
