#!/bin/bash
set -euo pipefail
room_file_name="$1"
room_file_path="$room_file_name"
new_message_offset="$(python -c '
import sys, os
with open(sys.argv[1], "rb") as f:
    f.seek(0, os.SEEK_END)
    consecutive_backslashes_count = 0
    for i in range(f.tell()-1, -1, -1):
        f.seek(i)
        c = f.read(1)
        if c == b"\n" and consecutive_backslashes_count == 1:
            next(f)
            break
        elif c == b"\\":
            consecutive_backslashes_count += 1
        else:
            consecutive_backslashes_count = 0
    print(f.tell())
' "$room_file_path")"
dd "if=$room_file_path" skip=$new_message_offset iflag=skip_bytes status=none | curl -s -f --data-binary @- -H "X-Room-File-Offset: $new_message_offset" -H "X-Room-File-Name: $room_file_name" "$blabber_url" | tee -a -- "$room_file_path"
