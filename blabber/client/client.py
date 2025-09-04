import sys, os, fcntl, urllib.request, base64, urllib.parse
room_file_path = sys.argv[1]
with open(room_file_path, "a+b") as room_file:
    fcntl.flock(room_file, flock.LOCK_EX)
    for i in range(room_file.tell()-1, -1, -1):
        room_file.seek(i)
        chunk = room_file.read(2)
        if chunk == b"\n\\" or (i == 0 and chunk.startswith(b"\\")):
            f.readline()
            break
    room_file_offset = room_file.tell()
    parse_result = urllib.parse(os.environ["blabber_url"])
    creds, real_netloc = parse_result.netloc.split("@")
    response = urllib.request.urlopen(urllib.request.Request("{scheme}://{real_netloc}/{room_file_name}".format(scheme=parse_result.scheme, real_netloc=real_netloc, room_file_name=os.path.basename(room_file_path)}), data=room_file, headers={"Range": "bytes={}-".format(new_message_offset), "Authorization": b"Basic " + base64.b64encode(creds)}))
    for chunk in iter(lambda: response.read(4096), b""):
        sys.stdout.buffer.write(chunk)
        room_file.write(chunk)
