#!/bin/bash

sudo dnf -y install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-28.noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-28.noarch.rpm

# OS update
sudo dnf update -y

sudo dnf -y install gcc-c++ ffmpeg

cd ~
curl https://static.rust-lang.org/dist/rust-1.31.0-x86_64-unknown-linux-gnu.tar.gz > rust-1.31.0-x86_64-unknown-linux-gnu.tar.gz
tar -zxvf rust-1.31.0-x86_64-unknown-linux-gnu.tar.gz
sudo ./rust-1.31.0-x86_64-unknown-linux-gnu/install.sh
