#!/bin/bash

# this script should be invoked from root directory,
# e.g. bash .github/workflows/render_test_scene.sh

red_start="\033[31m"
red_end="\033[0m"

for scene_file in ../pbrt-v4-scenes-json/*/*.json; do
    printf "rendering: ${red_start}${scene_file}${red_end}\n"
    ./target/release/pbrt-minus --spp=1 ${scene_file}
    printf " \n"
done
