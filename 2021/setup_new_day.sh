#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "please pass day number"
    exit 1
fi

mkdir $1
cd $1
cargo init --name solution$1
touch problem_part_1.md
touch problem_part_2.md
touch input.txt
