USAGE:

./ffmpeg_wraper <width> <height> <input_video_name>

Install:

A Vagrant file is provided. Simply run "vagrant up" after cloning the repo.
Run "vagrant ssh" to connect to the VM when it is up - and "vagrant destroy" to delete it later.
Once logged in, cd to /vagrant and run "./ffmpeg_wrapper <width> <height> <input_video>".

Example command, as run from /vagrant:

./ffmpeg 3 3 input_video.mp4
