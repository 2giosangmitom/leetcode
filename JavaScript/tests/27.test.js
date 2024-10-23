import { removeElement } from "../src/27.js";
import { assertEquals } from "jsr:@std/assert";

const cases = [
    { arr: [3, 2, 2, 3], val: 3, want: 2, wantArr: [2, 2] },
    {
        arr: [0, 1, 2, 2, 3, 0, 4, 2],
        val: 2,
        want: 5,
        wantArr: [0, 0, 1, 3, 4],
    },
];

Deno.test("27. Remove Element", () => {
    cases.forEach((i) => {
        const result = removeElement(i.arr, i.val);
        assertEquals(result, i.want);
        const postArr = i.arr.slice(0, i.want);
        postArr.sort();
        assertEquals(postArr, i.wantArr);
    });
});
