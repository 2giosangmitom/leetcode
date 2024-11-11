file(GLOB TEST_FILES "${CMAKE_SOURCE_DIR}/tests/*_test.cpp")

foreach(TEST_FILE ${TEST_FILES})
    get_filename_component(PROBLEM_NAME ${TEST_FILE} NAME_WE)
    string(REPLACE "_test" "" PROBLEM_NAME ${PROBLEM_NAME})

    set(SOURCE_FILE "${CMAKE_SOURCE_DIR}/src/${PROBLEM_NAME}.cpp")
    set(HEADER_FILE "${CMAKE_SOURCE_DIR}/src/${PROBLEM_NAME}.hpp")

    if (EXISTS ${SOURCE_FILE})
        add_executable(${PROBLEM_NAME}_test ${SOURCE_FILE} ${TEST_FILE})

        target_include_directories(${PROBLEM_NAME}_test PRIVATE ${CMAKE_SOURCE_DIR}/src)
        target_link_libraries(${PROBLEM_NAME}_test PRIVATE GTest::gtest_main)

        include(GoogleTest)
        gtest_discover_tests(${PROBLEM_NAME}_test)
    else()
        message(WARNING "Source file for ${PROBLEM_NAME} not found, skipping test executable.")
    endif()
endforeach()
