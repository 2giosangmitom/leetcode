import { assertEquals } from "@std/assert";
import { summaryRanges } from "./228.ts";

interface tt {
  nums: number[];
  want: string[];
}

const cases: tt[] = [
  { nums: [0, 1, 2, 4, 5, 7], want: ["0->2", "4->5", "7"] },
  { nums: [0, 2, 3, 4, 6, 8, 9], want: ["0", "2->4", "6", "8->9"] },
];

for (const t of cases) {
  Deno.test(JSON.stringify(t), () => {
    const result = summaryRanges(t.nums);
    assertEquals(result, t.want);
  });
}
