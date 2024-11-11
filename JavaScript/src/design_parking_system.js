/**
 * @param {number} big
 * @param {number} medium
 * @param {number} small
 */
class ParkingSystem {
  #big;
  #medium;
  #small;
  constructor(big, medium, small) {
    this.#big = big;
    this.#medium = medium;
    this.#small = small;
  }

  /**
   * @param {number} carType
   * @return {boolean}
   */
  addCar(carType) {
    if (carType === 1 && this.#big > 0) {
      this.#big--;
      return true;
    } else if (carType === 2 && this.#medium > 0) {
      this.#medium--;
      return true;
    } else if (carType === 3 && this.#small > 0) {
      this.#small--;
      return true;
    }
    return false;
  }
}

export { ParkingSystem };
