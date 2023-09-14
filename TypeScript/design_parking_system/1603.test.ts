import ParkingSystem from './1603';
import { describe, test, expect } from 'bun:test';

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

describe('parking system', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result: boolean[] = [];
      const obj = new ParkingSystem(t.big, t.medium, t.small);
      for (const type of t.addCar) {
        result.push(obj.addCar(type));
      }
      expect(result).toEqual(t.want);
    });
  });
});
