#!/usr/bin/env python3
import sys, os, fcntl, urllib.request, json
room_file_path = sys.argv[1]
with open(room_file_path, "a+") as room_file:
    fcntl.flock(room_file, fcntl.LOCK_EX)
    for i in range(room_file.tell()-1, -1, -1):
        room_file.seek(i)
        chunk = room_file.read(2)
        if chunk == "\n\\" or (i == 0 and chunk.startswith("\\")):
            room_file.readline()
            break
        else:
            room_file.seek(i)
    new_message_offset = room_file.tell()
    with urllib.request.urlopen(urllib.request.Request(os.environ["blabber_url"], data=(json.dumps({"offset": new_message_offset, "msg": room_file.read(), "room": os.path.basename(room_file_path), "password": os.environ["blabber_password"], "username": os.environ["blabber_username"]}) + "\n").encode("ascii"))) as response:
        for chunk in iter(lambda: response.read(4096), b""):
            sys.stdout.buffer.write(chunk)
            room_file.write(chunk.decode("utf-8"))
