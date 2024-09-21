CXX = clang++
CXXFLAGS = -std=c++17 -Wall -Iinclude

SOLUTIONS_DIR = src
TESTS_DIR = tests
INCLUDE_DIR = include

SOLUTION_SRCS = $(wildcard $(SOLUTIONS_DIR)/*.cpp)
TEST_SRCS = $(wildcard $(TESTS_DIR)/*.cpp)
SOLUTION_OBJS = $(SOLUTION_SRCS:.cpp=.o)
TEST_OBJS = $(TEST_SRCS:.cpp=.o)

TESTS = $(TEST_OBJS:.o=)

GTEST_INCS = /nix/store/*-gtest-*/include
GTEST_LIB = /nix/store/*-gtest-*/lib/libgtest.so
GTEST_MAIN_LIB = /nix/store/*-gtest-*/lib/libgtest_main.so
GTEST_FLAGS = -pthread

all: $(TESTS)

# Build test executables
$(TESTS): % : %.o $(SOLUTION_OBJS) $(GTEST_LIB) $(GTEST_MAIN_LIB)
	$(CXX) $(CXXFLAGS) -I$(GTEST_INCS) $< $(SOLUTION_OBJS) $(GTEST_LIB) $(GTEST_MAIN_LIB) -o $@ $(GTEST_FLAGS)

# Build object files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -I$(INCLUDE_DIR) -I$(GTEST_INCS) -c $< -o $@

test: all
	@for test in $(TESTS); do ./$$test; done

clean:
	rm -f $(SOLUTION_OBJS) $(TEST_OBJS) $(TESTS)

.PHONY: all test clean
