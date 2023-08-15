package parkingsystem

import (
	"fmt"
	"reflect"
	"testing"
)

func TestParkingSystem(t *testing.T) {
	tt := [...]struct {
		big    int
		medium int
		small  int
		addCar []int
		want   []bool
	}{
		{big: 1, medium: 1, small: 0, addCar: []int{1, 2, 3, 1}, want: []bool{true, true, false, false}},
		{big: 2, medium: 0, small: 3, addCar: []int{1, 1, 1, 2, 2, 3, 3, 3}, want: []bool{true, true, false, false, false, true, true, true}},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			obj := Constructor(test.big, test.medium, test.small)
			result := []bool{}
			for _, j := range test.addCar {
				add := obj.AddCar(j)
				result = append(result, add)
			}
			if !reflect.DeepEqual(result, test.want) {
				t.Errorf("Got %v but want %v", result, test.want)
			}
		})
	}
}
