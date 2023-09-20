#!/bin/bash

red_start="\033[31m"
red_end="\033[0m"

scene_dir="../pbrt-minus-scenes"
if [ ! -d "${scene_dir}" ]; then
    echo "please download scenes from ${red_start}https://github.com/w3ntao/pbrt-minus-scenes${red_end}"
    echo "to the same level as pbrt-minus"
    exit 1
fi

mode="$1"
if [ "$mode" = "--debug" ]; then
    mode="debug"
else
    mode="release"
fi

for scene_file in ${scene_dir}/*/*.json; do
    command="./target/${mode}/pbrt-minus --spp=1"
    printf "${command} ${red_start}${scene_file}${red_end}\n"
    eval "${command} ${scene_file}" || {
        echo 'fail'
        exit 1
    }
    printf " \n"
done
