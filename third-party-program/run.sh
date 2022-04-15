#!/usr/bin/bash

mkdir build
gcc "program.c" -Wall -g -o "./build/third-party-program"

cargo build

LD_PRELOAD="./../target/debug/libmetalbear_lib.so" ./build/third-party-program