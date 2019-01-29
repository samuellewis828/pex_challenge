#!/bin/bash


ffprobe -select_streams v:0 -skip_frame nokey -show_frames $3 > frames.txt

cd output_pictures
rm *
ffmpeg -skip_frame nokey -i ../$3 -vsync 0 -f image2 thumbnails-%01d.png
cd ../

cargo run -- --width $1 --height $2
