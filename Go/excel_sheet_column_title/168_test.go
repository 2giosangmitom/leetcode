package excelsheetcolumntitle

import "testing"

func Test_convertToTitle(t *testing.T) {
	type args struct {
		columnNumber int
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{name: "case 1", args: args{columnNumber: 1}, want: "A"},
		{name: "case 2", args: args{columnNumber: 28}, want: "AB"},
		{name: "case 3", args: args{columnNumber: 701}, want: "ZY"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := convertToTitle(tt.args.columnNumber); got != tt.want {
				t.Errorf("convertToTitle() = %v, want %v", got, tt.want)
			}
		})
	}
}
