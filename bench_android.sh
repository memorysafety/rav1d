#!/bin/bash

# Enable exit on error
set -e

for i in {1..10}
do
    echo "testing dav1d"
    adb shell LD_LIBRARY_PATH=/data/local/ time /data/local/dav1d_c_impl -q -i /data/local/Chimera-AV1-8bit-1920x1080-6736kbps.ivf -o /dev/null
    echo "testing rav1d"
    adb shell LD_LIBRARY_PATH=/data/local/ time /data/local/dav1d -q -i /data/local/Chimera-AV1-8bit-1920x1080-6736kbps.ivf -o /dev/null
done
