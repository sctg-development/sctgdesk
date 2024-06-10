#!/bin/bash
for size in 16 32 48 64 128 256 512 1024; do
    #inkscape -z -o $size.png -w $size -h $size icon.svg >/dev/null 2>/dev/null
    convert icon.png -resize ${size}x${size} app_icon_$size.png
done
# from ImageMagick
convert app_icon_16.png app_icon_32.png app_icon_48.png app_icon_128.png app_icon_256.png -colors 256 icon.ico
cp app_icon_32.png 32x32.png
cp app_icon_64.png 64x64.png
cp app_icon_128.png 128x128.png
cp app_icon_256.png 128x128@2x.png
#/bin/rm 16.png 32.png 48.png 128.png 256.png

convert app_icon_256.png ../flutter/macos/Runner/AppIcon.icns