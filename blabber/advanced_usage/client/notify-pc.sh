#!/bin/bash
# The dumbest way I found to make this thing. Just leave it running in the background
set -euo pipefail
while : ; do
    notification="$(./client.py "$@" | tail)"
    if [ "$notification" != "" ]; then
        notify-send -a blabber -- "$notification"
    fi
    sleep 5
done
