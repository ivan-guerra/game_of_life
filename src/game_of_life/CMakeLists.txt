cmake_minimum_required(VERSION 3.16)

add_executable(${CMAKE_PROJECT_NAME})

target_sources(${CMAKE_PROJECT_NAME} PRIVATE game_of_life.cpp)

target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE game graphics)

install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION "${GOL_BIN_DIR}")
