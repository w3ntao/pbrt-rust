#!/bin/bash

red_start="\033[31m"
red_end="\033[0m"

scene_dir="../pbrt-rust-scenes"
if [ ! -d "${scene_dir}" ]; then
    echo "please download scenes from ${red_start}https://github.com/w3ntao/pbrt-rust-scenes${red_end}"
    echo "to the same level as pbrt-rust"
    exit 1
fi

mode="$1"
if [ "$mode" = "--debug" ]; then
    mode="debug"
else
    mode="release"
fi

for scene_file in ${scene_dir}/*/*.pbrt; do
    command="./target/${mode}/pbrt-rust --spp=1"
    printf "${command} ${red_start}${scene_file}${red_end}\n"
    eval "${command} ${scene_file}" || {
        echo 'fail'
        exit 1
    }
    printf " \n"
done
