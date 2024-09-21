CXX = clang++
CXXFLAGS = -std=c++17 -Wall -I$(INCLUDE_DIR)
LDFLAGS = -pthread

# Directories
SOLUTIONS_DIR = src
TESTS_DIR = tests
INCLUDE_DIR = include

# Google Test Paths
GTEST_ROOT = $(GTEST_PKG)
GTEST_INCS = $(GTEST_ROOT)/include
GTEST_LIB = $(GTEST_ROOT)/lib/libgtest.so
GTEST_MAIN_LIB = $(GTEST_ROOT)/lib/libgtest_main.so

# Source files and object files
SOLUTION_SRCS = $(wildcard $(SOLUTIONS_DIR)/*.cpp)
TEST_SRCS = $(wildcard $(TESTS_DIR)/*.cpp)
SOLUTION_OBJS = $(SOLUTION_SRCS:.cpp=.o)
TEST_OBJS = $(TEST_SRCS:.cpp=.o)

# Test executables
TESTS = $(TEST_OBJS:.o=)

# Default target
all: $(TESTS)

# Rule to build test executables
$(TESTS): % : %.o $(SOLUTION_OBJS)
	$(CXX) $(CXXFLAGS) -I$(GTEST_INCS) $< $(SOLUTION_OBJS) $(GTEST_LIB) $(GTEST_MAIN_LIB) -o $@ $(LDFLAGS)

# Pattern rule to build object files for both solutions and tests
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -I$(GTEST_INCS) -c $< -o $@

# Run all tests
test: all
	@for test in $(TESTS); do \
		echo "Running $$test..."; \
		./$$test; \
	done

# Debug build
debug: CXXFLAGS += -g
debug: all

# Clean up
clean:
	rm -f $(SOLUTION_OBJS) $(TEST_OBJS) $(TESTS)

.PHONY: all test debug clean
