import { ParkingSystem } from "#src/design_parking_system";
import { expect, test } from "vitest";

const cases = [
  {
    name: "no small slots",
    big: 1,
    medium: 1,
    small: 0,
    addCars: [1, 2, 3, 1],
    want: [true, true, false, false],
  },
  {
    name: "all slots full",
    big: 1,
    medium: 1,
    small: 1,
    addCars: [1, 2, 3, 1, 2, 3],
    want: [true, true, true, false, false, false],
  },
  {
    name: "only big slots",
    big: 2,
    medium: 0,
    small: 0,
    addCars: [1, 1, 2, 3],
    want: [true, true, false, false],
  },
  {
    name: "only medium slots",
    big: 0,
    medium: 2,
    small: 0,
    addCars: [2, 2, 1, 3],
    want: [true, true, false, false],
  },
  {
    name: "only small slots",
    big: 0,
    medium: 0,
    small: 2,
    addCars: [3, 3, 1, 2],
    want: [true, true, false, false],
  },
  {
    name: "mixed slot capacities",
    big: 2,
    medium: 1,
    small: 1,
    addCars: [1, 1, 2, 3, 1],
    want: [true, true, true, true, false],
  },
];

for (const c of cases) {
  test(c.name, () => {
    const ps = new ParkingSystem(c.big, c.medium, c.small);
    for (let i = 0; i < c.addCars.length; i++) {
      expect(ps.addCar(c.addCars[i])).toBe(c.want[i]);
    }
  });
}
