#!/bin/bash

CWD=$(pwd)

# Root directory.
GOL_PROJECT_PATH=$(dirname ${CWD})

# Binary directory.
GOL_BIN_DIR="${GOL_PROJECT_PATH}/bin"

# Resource directory.
GOL_RES_DIR="${GOL_PROJECT_PATH}/resources"

# CMake build files and cache.
GOL_BUILD_DIR="${GOL_PROJECT_PATH}/build"
