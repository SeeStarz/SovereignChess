#!/usr/bin/sh
run=$(realpath $(dirname $0))/dll
export LD_LIBRARY_PATH=${run}
cd $run 
./main ../config/config.cfg
