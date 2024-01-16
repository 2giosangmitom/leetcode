cmake_minimum_required(VERSION 3.14)
project(leetcode)

include_directories(src)

# GoogleTest requires at least C++14
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/tags/v1.14.0.zip
  DOWNLOAD_EXTRACT_TIMESTAMP true
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()
file(GLOB SOURCES "src/*.cpp")

add_executable(testLeetcode ${SOURCES})
target_link_libraries(
  testLeetcode
  GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(testLeetcode)
