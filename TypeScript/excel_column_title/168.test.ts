import convertToTitle from "./168.ts";
import { assertEquals } from "../deps.ts";

interface tt {
  columnNumber: number;
  want: string;
}

const cases: tt[] = [
  { columnNumber: 1, want: "A" },
  { columnNumber: 28, want: "AB" },
  { columnNumber: 701, want: "ZY" },
];

for (const t of cases) {
  Deno.test("excel column title", () => {
    const result = convertToTitle(t.columnNumber);
    assertEquals(result, t.want);
  });
}
