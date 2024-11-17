# Glob all test files matching *_test.cpp in the tests directory
file(GLOB TEST_FILES "${CMAKE_SOURCE_DIR}/tests/*_test.cpp")

foreach(TEST_FILE IN LISTS TEST_FILES)
  # Extract the problem name by removing the directory and extension
  get_filename_component(TEST_NAME ${TEST_FILE} NAME_WE)
  string(REPLACE "_test" "" PROBLEM_NAME ${TEST_NAME})

  # Define expected source and header file paths
  set(SOURCE_FILE "${CMAKE_SOURCE_DIR}/src/${PROBLEM_NAME}.cpp")
  set(HEADER_FILE "${CMAKE_SOURCE_DIR}/src/${PROBLEM_NAME}.hpp")

  # Check if the source file exists
  if (EXISTS ${SOURCE_FILE})
    # Create a test executable
    add_executable(${TEST_NAME} ${SOURCE_FILE} ${TEST_FILE})

    # Include the source directory and link the Google Test library
    target_include_directories(${TEST_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/src)
    target_link_libraries(${TEST_NAME} PRIVATE GTest::gtest_main)

    # Automatically discover Google Test cases
    include(GoogleTest)
    gtest_discover_tests(${TEST_NAME})
  else()
    # Warn if the source file does not exist
    message(WARNING "Source file ${SOURCE_FILE} for test ${TEST_NAME} not found. Skipping.")
  endif()
endforeach()
