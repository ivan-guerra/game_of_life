cmake_minimum_required(VERSION 3.16)

project(life DESCRIPTION "a ncurses visualization of Conway's game of life")

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake)

set(GOL_BIN_DIR
    "${CMAKE_SOURCE_DIR}/bin"
    CACHE STRING "${PROJECT_NAME} binary directory.")
set(GOL_INCLUDE_DIR
    "${CMAKE_SOURCE_DIR}/include"
    CACHE STRING "${PROJECT_NAME} include directory.")

include(FetchContent)

set(COMMON_FLAGS -std=c++2a)

set(WARNING_FLAGS -Wall -Wextra -Werror -pedantic)

set(DEBUG_FLAGS
    ${COMMON_FLAGS}
    ${WARNING_FLAGS}
    -O0
    -g3
    -ggdb
    -fno-omit-frame-pointer
    -fsanitize=address)

set(RELEASE_FLAGS ${COMMON_FLAGS} -O2)

add_link_options("$<$<CONFIG:Debug>:-fsanitize=address>")
add_compile_options("$<$<CONFIG:Release>:${RELEASE_FLAGS}>"
                    "$<$<CONFIG:Debug>:${DEBUG_FLAGS}>")

add_subdirectory(docs)
add_subdirectory(src)
