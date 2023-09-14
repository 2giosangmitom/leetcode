package parkingsystem

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParkingSystem(t *testing.T) {
	tests := []struct {
		big    int
		medium int
		small  int
		addCar []int
		want   []bool
	}{
		{
			big:    1,
			medium: 1,
			small:  0,
			addCar: []int{1, 2, 3, 1},
			want:   []bool{true, true, false, false},
		},
		{
			big:    2,
			medium: 0,
			small:  3,
			addCar: []int{1, 1, 1, 2, 2, 3, 3, 3},
			want:   []bool{true, true, false, false, false, true, true, true},
		},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			obj := Constructor(tt.big, tt.medium, tt.small)
			result := []bool{}
			for _, j := range tt.addCar {
				add := obj.AddCar(j)
				result = append(result, add)
			}
			assert.Equal(t, tt.want, result)
		})
	}
}
