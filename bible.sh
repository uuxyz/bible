#!/bin/bash
if [ $# -eq 1 ]; then
    ./bible 15 $1
elif [ $# -eq 2 ]; then
    start=$1
    end=$2
    ./bible 14 $start
    echo -n " - "
    ./bible 14 $end
    echo ""
    for ((i=start; i<=end; i++)); do
        ./bible 1 $i
        echo -n " "
    done
    echo ""
else
    ./bible 15 $RANDOM
fi


