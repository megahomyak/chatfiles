#!/bin/bash
set -euo pipefail
room_file_name="$1"
room_file_path="$room_file_name"
new_message_offset="$(python -c '
import sys, os
with open(sys.argv[1], "rb") as f:
    f.seek(0, os.SEEK_END)
    for i in range(f.tell()-1, -1, -1):
        f.seek(i)
        chunk = f.read(2)
        if chunk == b"\n\\" or (i == 0 and chunk.startswith(b"\\")):
            next(f)
            break
    print(f.tell())
' "$room_file_path")"
exec {lock_fd}> "$room_file_path"
flock -x "$lock_fd"
dd "if=$room_file_path" skip=$new_message_offset iflag=skip_bytes status=none | curl -s -f --data-binary @- -H "X-Room-File-Offset: $new_message_offset" -H "X-Room-File-Name: $room_file_name" "$blabber_url" | tee -a -- "$room_file_path"
