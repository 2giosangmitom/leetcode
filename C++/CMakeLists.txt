cmake_minimum_required(VERSION 3.29)
project(leetcode)

# Set C++ standard to C++23
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set compiler flags
add_compile_options(-Wall -Wextra -g)

# Enable compile commands for IDEs
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Enable testing using GTest
find_package(GTest REQUIRED)
enable_testing()

# Add test subdirectories
add_subdirectory(tests)
