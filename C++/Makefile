generate:
	CC="zig cc" CXX="zig c++" cmake -S . -B build

build:
	cmake --build build

test:
	(cd build/ && make test)

.PHONY: generate build
