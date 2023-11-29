#!/bin/bash

source config.sh

# Remove the binary directory.
if [ -d $GOL_BIN_DIR ]
then
    echo "removing '$GOL_BIN_DIR'"
    rm -r $GOL_BIN_DIR
fi

# Remove the CMake build directory.
if [ -d $GOL_BUILD_DIR ]
then
    echo "removing '$GOL_BUILD_DIR'"
    rm -rf $GOL_BUILD_DIR
fi
