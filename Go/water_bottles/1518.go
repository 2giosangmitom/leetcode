package waterbottles

func numWaterBottles(numBottles int, numExchange int) int {
	drank := 0
	for numBottles >= numExchange {
		rem := numBottles % numExchange
		drank += numBottles - rem
		numBottles = (numBottles / numExchange) + rem
	}
	return drank + numBottles
}
