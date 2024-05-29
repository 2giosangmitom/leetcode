import { createList } from "../lib.ts";
import { addTwoNumbers } from "../src/add_two_numbers.ts";
import { assertEquals } from "@std/assert";

const cases = [
  {
    l1: createList([2, 4, 3]),
    l2: createList([5, 6, 4]),
    want: createList([7, 0, 8]),
  },
  {
    l1: createList([0]),
    l2: createList([0]),
    want: createList([0]),
  },
  {
    l1: createList([9, 9, 9, 9, 9, 9, 9]),
    l2: createList([9, 9, 9, 9]),
    want: createList([8, 9, 9, 9, 0, 0, 0, 1]),
  },
];

for (let i = 0; i < cases.length; i++) {
  const t = cases[i];

  Deno.test(`addTwoNumbers - test case ${i + 1}`, () => {
    const result = addTwoNumbers(t.l1, t.l2);
    assertEquals(result, t.want);
  });
}
