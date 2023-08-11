/**
 * Runtime: 1ms (Beats 80.95%)
 * Memory: 1.9MB (Beats 100%)
 */

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
