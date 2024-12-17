cmake_minimum_required(VERSION 3.29)
project(leetcode)

# Set C++ standard to C++23
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Enable compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add compiler warnings and debugging information
add_compile_options(-Wall -Wextra -g)

# Find and enable GoogleTest for testing
find_package(GTest REQUIRED)
enable_testing()

# Include the source directory for headers
include_directories(${CMAKE_SOURCE_DIR}/src)

# Glob all test files matching *_test.cpp in the tests directory
file(GLOB TEST_FILES "${CMAKE_SOURCE_DIR}/tests/*_test.cpp")

# Iterate over each test file
foreach(TEST_FILE IN LISTS TEST_FILES)
  # Extract the base name of the test file (without extension)
  get_filename_component(TEST_NAME ${TEST_FILE} NAME_WE)
  
  # Derive the problem name by removing the "_test" suffix
  string(REPLACE "_test" "" PROBLEM_NAME ${TEST_NAME})
  set(HEADER_FILE "${CMAKE_SOURCE_DIR}/src/${PROBLEM_NAME}.hpp")

  # Only proceed if the corresponding header file exists
  if (EXISTS "${HEADER_FILE}")
    # Create an executable for the test
    add_executable(${TEST_NAME} ${HEADER_FILE} ${TEST_FILE})
    target_include_directories(${TEST_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/src)

    # Link GoogleTest to the test executable
    target_link_libraries(${TEST_NAME} PRIVATE GTest::gtest_main)

    # Discover GoogleTest tests
    include(GoogleTest)
    gtest_discover_tests(${TEST_NAME})
  else()
    # Warn if the header file for the test doesn't exist
    message(WARNING "Header file ${HEADER_FILE} for test ${TEST_NAME} not found. Skipping.")
  endif()
endforeach()
