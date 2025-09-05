import sys, os, fcntl, urllib.request, base64, urllib.parse, json
room_file_path = sys.argv[1]
with open(room_file_path, "a+") as room_file:
    fcntl.flock(room_file, fcntl.LOCK_EX)
    for i in range(room_file.tell()-1, -1, -1):
        room_file.seek(i)
        chunk = room_file.read(2)
        if chunk == "\n\\" or (i == 0 and chunk.startswith("\\")):
            f.readline()
            break
    new_message_offset = room_file.tell()
    parse_result = urllib.parse.urlparse(os.environ["blabber_url"])
    creds, real_netloc = parse_result.netloc.split("@")
    with urllib.request.urlopen(urllib.request.Request("{scheme}://{real_netloc}/{room_file_name}".format(scheme=parse_result.scheme, real_netloc=real_netloc, room_file_name=os.path.basename(room_file_path)), data=json.dumps({"offset": new_message_offset, "new_message": room_file.read(), }), headers={"Range": "bytes={}-".format(new_message_offset), "Authorization": b"Basic " + base64.b64encode(creds.encode())})) as response:
        for chunk in iter(lambda: response.read(4096), b""):
            sys.stdout.buffer.write(chunk)
            room_file.write(chunk)
