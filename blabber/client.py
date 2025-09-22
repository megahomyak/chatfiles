#!/usr/bin/env python3
import sys, os, urllib.request, urllib.error, io, base64
room_file_path = sys.argv[1]
with open(room_file_path, "a+b") as room_file:
    end_offset = room_file.tell()
    for i in range(end_offset - 1, -1, -1):
        room_file.seek(i)
        chunk = room_file.read(2)
        if chunk == b"\n\\" or (i == 0 and chunk.startswith(b"\\")):
            room_file.readline()
            break
        else:
            room_file.seek(i)
    new_message_offset = room_file.tell()
    try:
        with urllib.request.urlopen(urllib.request.Request(os.environ["blabber_url"] + "/rooms/" + os.path.basename(room_file_path), headers={"Range": "bytes={}-".format(new_message_offset), "Content-Length": end_offset - new_message_offset, "Authorization": b"Basic " + base64.b64encode(os.environ["blabber_creds"].encode("utf-8"))}, data=room_file)) as response:
            for chunk in iter(lambda: response.read(4096), b""):
                sys.stdout.buffer.write(chunk)
                room_file.write(chunk)
    except urllib.error.HTTPError as error:
        raise Exception(str(error.code) + " : " + error.reason + " : " + error.file.read().decode())
