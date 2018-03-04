#!/bin/sh
while true; do
    inotifywait -q --event modify slides.adoc
    make
done
