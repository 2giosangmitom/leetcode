import { singleNumber, singleNumber2 } from "../src/single_number.js";
import { assertEquals } from "@std/assert";

const cases = [
  { name: "same numbers twice", nums: [2, 2, 1], want: 1 },
  { name: "same numbers interleaved", nums: [4, 1, 2, 1, 2], want: 4 },
  { name: "one element", nums: [1], want: 1 },
];

for (const tt of cases) {
  Deno.test(tt.name, () => {
    assertEquals(
      singleNumber(tt.nums),
      tt.want,
      "implementation 1 (bit manipulation)",
    );
    assertEquals(singleNumber2(tt.nums), tt.want, "implementation 2 (sort)");
  });
}
