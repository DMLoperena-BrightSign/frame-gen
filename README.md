# Frame-Gen

Frame-Gen ( or frame-gen ) is a command-line program meant to be used to create videos.
It works by generating a series of frames given the parameters passed in by the user
and these frames can then be compiled into a video using something like ffmpeg.

Below is an example command that takes the frames generated by this program and creates
an mp4 with the current frame number overlayed.

```
ffmpeg -r 60 -i frame%d.png -c:v libx265 -vf "drawtext=fontfile=/usr/share/fonts/truetype/dejavu/DejaVuSans-BoldOblique.ttf: text='Frame\: %{frame_num}': start_number=1: x=(w-tw)/2: y=h-(2*1h): fontcolor=black: fontsize=200,fps=60,format=yuv420p" -minrate 10M -maxrate 20M -b:v 25M -x265-params pass=1 -f mp4 /dev/null && ffmpeg -r 60 -i frame%d.png -c:v libx265 -vf "drawtext=fontfile=/usr/share/fonts/truetype/dejavu/DejaVuSans-BoldOblique.ttf: text='Frame\: %{frame_num}': start_number=1: x=(w-tw)/2: y=h-(2*1h): fontcolor=black: fontsize=200,fps=60,format=yuv420p" -minrate 10M -maxrate 20M -b:v 25M -x265-params pass=2 out.mp4
```
![alt text](https://github.com/DMLoperena-BrightSign/frame-gen/blob/main/example/output.gif)
