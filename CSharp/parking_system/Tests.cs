namespace parking_system;

public class Test_ParkingSystem {
  [Fact]
  public void Test1() {
    var parkingSystem = new ParkingSystem(1, 1, 0);
    Assert.True(parkingSystem.AddCar(1));
    Assert.True(parkingSystem.AddCar(2));
    Assert.False(parkingSystem.AddCar(3));
    Assert.False(parkingSystem.AddCar(1));
  }
}
