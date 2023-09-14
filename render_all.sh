#!/bin/bash

# you should have pbrt-minus-scene (https://github.com/w3ntao/pbrt-minus-scene)
# cloned in the same level as pbrt-minus before running this script

red_start="\033[31m"
red_end="\033[0m"

for scene_file in ../pbrt-minus-scene/*/*.json; do
    printf "rendering: ${red_start}${scene_file}${red_end}\n"
    ./target/release/pbrt-minus --spp=1 ${scene_file}
    printf " \n"
done
