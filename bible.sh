#!/bin/bash
if [ -n "$1" ]; then
    random_verse=$(($1 % 31102))
else
    random_verse=$(($RANDOM % 31102))
fi

./bible $random_verse