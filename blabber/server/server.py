#!/usr/bin/python3
import os, base64, fcntl, http.server, hashlib, datetime, shutil, sys, getpass
with open(os.environ["blabber_credfile"], "a+") as credfile:
    if sys.argv[1] == "serve":
        credfile.seek(0)
        password_lines = credfile.read().splitlines()
        class Handler(http.server.BaseHTTPRequestHandler):
            def do_POST(self):
                username, password = base64.b64decode(self.headers["Authorization"].split("Basic", 1)[1].lstrip()).decode().split(":")
                assert "{username}:{password_hash}".format(username=username, password_hash=hashlib.sha256(password.encode()).hexdigest()) in password_lines
                room_file_offset = int(self.headers["Range"].split("bytes=", 1)[1].split("-")[0])
                room_file_path = os.path.basename(self.path.split("/", 1)[1])
                print(1)
                while True:
                    print(self.rfile.read(1))
                new_message = self.rfile.read()
                print(2)
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
        host, port = os.environ["blabber_bind"].split(":")
        http.server.HTTPServer((host, int(port)), Handler).serve_forever()
    elif sys.argv[1] == "add_user":
        credfile.write("{username}:{password}".format(username=sys.argv[2], password=hashlib.sha256(getpass.getpass().encode()).hexdigest()))
    else: raise Exception("Unknown command \"{}\"".format(sys.argv[1]))
