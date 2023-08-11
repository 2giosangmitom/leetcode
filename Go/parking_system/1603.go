/**
 * Runtime: 35ms (Beats 87.33%)
 * Memory: 7.8MB (Beats 32.67%)
 */

package parkingsystem

type ParkingSystem struct {
	small  int
	medium int
	big    int
}

func Constructor(big int, medium int, small int) ParkingSystem {
	return ParkingSystem{small, medium, big}
}

func (p *ParkingSystem) AddCar(carType int) bool {
	if carType == 1 && p.big > 0 {
		p.big--
		return true
	}
	if carType == 2 && p.medium > 0 {
		p.medium--
		return true
	}
	if carType == 3 && p.small > 0 {
		p.small--
		return true
	}
	return false
}
