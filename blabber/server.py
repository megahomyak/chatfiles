#!/usr/bin/env python3
import os, fcntl, http.server, hashlib, datetime, shutil, sys, getpass, base64
with open(os.environ["blabber_credfile"], "a+b") as credfile:
    if sys.argv[1] == "serve":
        credfile.seek(0)
        password_lines = set(credfile.read().splitlines())
        class Handler(http.server.BaseHTTPRequestHandler):
            def send_response_with_headers(self, code):
                self.send_response(code)
                self.end_headers()
            def do_POST(self):
                try:
                    msg = self.rfile.read(int(self.headers["Content-Length"]))
                    username, password = base64.b64decode(self.headers["Authorization"].split("Basic ", 1)[1]).split(b":")
                    room_name = self.path.split("/rooms/", 1)[1]
                    assert username + b":" + hashlib.sha256(password).hexdigest().encode("ascii") in password_lines
                    assert room_name == os.path.basename(room_name)
                    assert all(not line.startswith(b"\\") for line in msg.splitlines())
                    self.send_response_with_headers(200)
                    footer = b"\n\\" + username + b" @ " + str(datetime.datetime.now(datetime.timezone.utc)).encode("ascii") + b"\n"
                    if msg: self.wfile.write(footer)
                    try:
                        with open(room_name, "a+b" if msg else "rb") as room_file:
                            fcntl.flock(room_file, fcntl.LOCK_EX)
                            room_file.seek(int(self.headers["Range"].split("bytes=", 1)[1].split("-", 1)[0]))
                            shutil.copyfileobj(room_file, self.wfile)
                            if msg: room_file.write(msg + footer)
                    except FileNotFoundError: pass
                except AssertionError: self.send_response_with_headers(400)
                except: self.send_response_with_headers(500)
        http.server.HTTPServer((os.environ["blabber_host"], int(os.environ["blabber_port"])), Handler).serve_forever()
    elif sys.argv[1] == "add_user":
        credfile.write("{username}:{password}\n".format(username=sys.argv[2], password=hashlib.sha256(getpass.getpass().encode("utf-8")).hexdigest()).encode("utf-8"))
    else: raise Exception("Unknown command \"{}\"".format(sys.argv[1]))
