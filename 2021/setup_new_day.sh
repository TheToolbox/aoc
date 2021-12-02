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
wget -O input.txt https://adventofcode.com/2021/day/$1/input
