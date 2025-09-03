#!/bin/bash
set -euo pipefail
room_file_name="$1"
room_file_path="$room_file_name"
new_message_offset="$(python -c '
import sys, os, urllib.request, base64
blabber_url = os.environ["blabber_url"]
room_file_name = sys.argv[1]
room_file_path = room_file_name
with open(sys.argv[1], "r+b") as f:
    f.seek(0, os.SEEK_END)
    for i in range(f.tell()-1, -1, -1):
        f.seek(i)
        chunk = f.read(2)
        if chunk == b"\n\\" or (i == 0 and chunk.startswith(b"\\")):
            next(f)
            break
    new_message_offset = f.tell()
    credentials = blabber_url
    with urllib.request.urlopen(urllib.request.Request(
        blabber_url,
        data=f,
        headers={
            "Range": f"bytes={new_message_offset}-",
            "Authorization": f"Basic {base64.b64encode(credentials.encode("utf-8")).decode("ascii")}"
        },
    )) as response:

' "$room_file_path")"
dd "if=$room_file_path" skip=$new_message_offset iflag=skip_bytes status=none | curl -s -f --data-binary @- -H "X-Room-File-Offset: $new_message_offset" -H "X-Room-File-Name: $room_file_name" "$blabber_url" | tee -a -- "$room_file_path"
