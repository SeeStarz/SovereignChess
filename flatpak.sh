#!/usr/bin/sh
run=$(realpath $(dirname $0))/../lib
export LD_LIBRARY_PATH=${run}
cd $run
./main
