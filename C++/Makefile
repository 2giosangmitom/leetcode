generate:
	CC=/usr/bin/clang CXX=/usr/bin/clang++ cmake -S . -B build

build:
	cmake --build build

test:
	(cd build/ && make test)

.PHONY: generate build
