#!/bin/bash
set -euo pipefail
room_file_name="$1"
room_file_path="$room_file_name"
touch -- "$room_file_path"
room_file_offset=$(( $(stat -c%s -- "$room_file_path") - $(tac -- "$room_file_path" | awk '/^\\[^\\]/ {exit} {print}' | wc --bytes) ))
dd "if=$room_file_path" skip=$room_file_offset iflag=skip_bytes status=none | curl -s -f --data-binary @- -H "X-Room-File-Offset: $room_file_offset" -H "X-Room-File-Name: $room_file_name" "$blabber_url" | tee -a -- "$room_file_path"
