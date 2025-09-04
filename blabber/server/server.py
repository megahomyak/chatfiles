#!/usr/bin/python3
import os, base64, fcntl, http.server, hashlib, datetime, shutil
password_lines = open(os.environ["blabber_credfile"]).read().splitlines()
class Handler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        username, password = base64.b64decode(self.headers["Authorization"].split("Basic", 1)[1]).decode().split(":")
        assert "{username}:{password_hash}".format(username=username, password_hash=hashlib.sha256(password.encode()).hexdigest()) in password_lines
        room_file_offset = int(self.headers["Range"].split("bytes=", 1)[1].split("-")[0])
        room_file_path = os.path.basename(self.path.split("/", 1)[1])
        new_message = self.rfile.read()
        assert all(not line.startswith(b"\\") for line in new_message.splitlines())
        self.send_response(200)
        self.end_headers()
        footer = "\n\\{username} @ {datetime}\n".format(username=username, datetime=datetime.datetime.now(datetime.timezone.utc)).encode()
        if new_message: self.wfile.write(footer)
        with open(room_file_path, "a+" if new_message else "r") as room_file: # This may throw, that's intended
            fcntl.flock(room_file, flock.LOCK_EX)
            if new_message: self.wfile.write(footer)
            room_file.seek(room_file_offset)
            shutil.copyfileobj(room_file_path, self.wfile)
            if new_message: room_file_path.write(new_message + footer)
http.server.HTTPServer((os.environ["blabber_host"], int(os.environ["blabber_port"])), Handler).serve_forever()
