namespace parking_system {
    public class ParkingSystem {
        int big, medium, small;
        public ParkingSystem(int big, int medium, int small) {
            this.big = big;
            this.medium = medium;
            this.small = small;
        }

        public bool AddCar(int carType) {
            return carType switch {
                1 => big-- > 0,
                2 => medium-- > 0,
                3 => small-- > 0,
                _ => false,
            };
        }
    }
}