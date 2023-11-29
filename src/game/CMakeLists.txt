cmake_minimum_required(VERSION 3.16)

project(
  game
  DESCRIPTION "game of life game logic"
  LANGUAGES CXX)

add_library(${PROJECT_NAME} STATIC)

target_sources(${PROJECT_NAME} PRIVATE board.cpp)

target_include_directories(${PROJECT_NAME} PUBLIC ${GOL_INCLUDE_DIR})
