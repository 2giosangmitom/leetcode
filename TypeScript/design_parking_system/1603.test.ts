import { assertEquals } from "../deps.ts";
import ParkingSystem from "./1603.ts";

interface tt {
  big: number;
  medium: number;
  small: number;
  addCar: number[];
  want: boolean[];
}

const tests: tt[] = [
  {
    big: 1,
    medium: 1,
    small: 0,
    addCar: [1, 2, 3, 1],
    want: [true, true, false, false],
  },
];

Deno.test("design parking system", () => {
  for (const t of tests) {
    const result: boolean[] = [];
    const obj = new ParkingSystem(t.big, t.medium, t.small);
    for (const type of t.addCar) {
      result.push(obj.addCar(type));
    }
    assertEquals(result, t.want);
  }
});
