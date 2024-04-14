package design_parking_system

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParkingSystem(t *testing.T) {
	cases := []struct {
		big     int
		medium  int
		small   int
		add_car []int
		want    []bool
	}{
		{big: 1, medium: 1, small: 0, add_car: []int{1, 2, 3, 1}, want: []bool{true, true, false, false}},
		{big: 2, medium: 15, small: 44, add_car: []int{1, 1, 2, 1, 3, 3, 1, 2, 2, 3, 1}, want: []bool{true, true, true, false, true, true, false, true, true, true, false}},
		{big: 1, medium: 1, small: 1, add_car: []int{1, 3, 3, 1}, want: []bool{true, true, false, false}},
		{big: 2, medium: 2, small: 2, add_car: []int{2, 1, 3, 3, 1, 2}, want: []bool{true, true, true, true, true, true}},
	}

	for i, tc := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			parkingSystem := Constructor(tc.big, tc.medium, tc.small)
			result := []bool{}
			for _, carType := range tc.add_car {
				result = append(result, parkingSystem.AddCar(carType))
			}
			assert.Equal(t, tc.want, result)
		})
	}
}
